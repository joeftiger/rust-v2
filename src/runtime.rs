use crate::renderer::Renderer;
use crate::util::threadpool::Threadpool;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use signal_hook::consts as signals;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{fs, thread};

#[derive(Clone)]
pub struct Runtime {
    pub renderer: Arc<Renderer>,
    pub tile_progress: Arc<AtomicUsize>,
    pub tiles: usize,
    pub total_tiles: usize,
    pub passes: usize,
    pub cancel: Arc<AtomicBool>,
    threadpool: Arc<Threadpool>,
}

impl Runtime {
    #[cold]
    pub fn new(renderer: Arc<Renderer>, tile_progress: Option<Arc<AtomicUsize>>) -> Self {
        let tile_progress = tile_progress.unwrap_or_else(|| Arc::new(AtomicUsize::new(0)));
        let tiles = renderer.sensor().num_tiles();
        let passes = renderer.config.passes;
        let total_tiles = tiles * passes;
        let cancel = Arc::new(AtomicBool::new(false));
        let threads = renderer.config.threads.unwrap_or_else(num_cpus::get);
        let c = cancel.clone();
        let threadpool = Arc::new(Threadpool::new(
            threads,
            None,
            Some(Box::new(move || c.store(true, Ordering::Relaxed))),
        ));

        log::info!(target: "Runtime", "Creating runtime with {} threads", threads);
        let runtime = Self {
            renderer,
            tile_progress,
            tiles,
            total_tiles,
            passes,
            cancel,
            threadpool,
        };
        runtime.init();
        runtime
    }

    fn init(&self) {
        self.progress_printer();
        self.cancel_signal();
        self.save_signal();
        self.checkpoint_signal();
    }

    fn progress_printer(&self) {
        let cancel = self.cancel.clone();
        let tile_progress = self.tile_progress.clone();
        let tiles = self.tiles;
        let passes = self.passes;

        thread::spawn(move || {
            let start = Instant::now();
            let mut counter = 0;

            while !cancel.load(Ordering::Relaxed) {
                let progress = tile_progress.load(Ordering::SeqCst);

                let pass = progress / tiles;
                let speed = pass as f64 / start.elapsed().as_secs_f64();
                let remaining_passes = passes - pass;
                let remaining = Duration::from_secs((remaining_passes as f64 / speed) as u64);

                let to_print = format!(
                    "{}/{}  {:.1}/s  {}m remaining",
                    pass,
                    passes,
                    speed,
                    remaining.as_secs() / 60
                );
                if counter % 10 == 0 {
                    log::info!(target: "Render passes", "{}", to_print);
                } else {
                    log::trace!(target: "Render passes", "{}", to_print);
                }
                counter += 1;

                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    fn cancel_signal(&self) {
        if let Err(e) = signal_hook::flag::register(signals::SIGINT, self.cancel.clone()) {
            log::warn!(target: "Runtime", "unable to create signal watcher for SIGINT: {}", e);
        }
        if let Err(e) = signal_hook::flag::register(signals::SIGTERM, self.cancel.clone()) {
            log::warn!(target: "Runtime", "unable to create signal watcher for SIGTERM: {}", e);
        }
    }

    fn save_signal(&self) {
        let save_flag = Arc::new(AtomicBool::new(false));

        if let Err(e) = signal_hook::flag::register(signals::SIGUSR1, save_flag.clone()) {
            log::warn!(target: "Runtime", "unable to register SIGUSR1 for saving rendering: {}", e);
        } else {
            let cancel = self.cancel.clone();
            let r = self.renderer.clone();
            let p = self.tile_progress.clone();
            let tiles = self.tiles;

            thread::spawn(move || {
                while !cancel.load(Ordering::Relaxed) {
                    if save_flag.fetch_and(false, Ordering::Relaxed) {
                        r.save_image(Some(p.load(Ordering::Relaxed) / tiles));
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });
            log::info!(target: "Runtime", "registered SIGUSR1 for saving rendering");
        }
    }

    fn checkpoint_signal(&self) {
        let checkpoint_flag = Arc::new(AtomicBool::new(false));

        if let Err(e) = signal_hook::flag::register(signals::SIGUSR2, checkpoint_flag.clone()) {
            log::warn!(target: "Runtime", "unable to register SIGUSR2 for saving checkpoint: {}", e);
        } else {
            let cancel = self.cancel.clone();
            let serde = RuntimeSerde {
                tile_progress: self.tile_progress.clone(),
                renderer: self.renderer.clone(),
            };
            let path = format!("{}.bin", &self.renderer.config.output);

            thread::spawn(move || {
                while !cancel.load(Ordering::Relaxed) {
                    if checkpoint_flag.fetch_and(false, Ordering::Relaxed) {
                        serde.save_to(&path);
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });
            log::info!(target: "Runtime", "registered SIGUSR2 for saving checkpoint");
        }
    }

    /// Loads either a RON or checkpointed [Runtime] from the given path.
    #[cold]
    pub fn load(path: &str) -> Option<Self> {
        match path.rsplit_once('.') {
            Some((_, "ron")) => Self::load_ron(path),
            Some((_, "bin")) => Self::load_checkpoint(path),
            Some((_, ending)) => {
                log::warn!(target: "Loading Runtime", "Unknown file ending: {}, trying best-effort", ending);
                Self::load_ron(path).or_else(|| Self::load_checkpoint(path))
            }
            None => {
                log::warn!(target: "Loading Runtime", "Unknown file type, trying best-effort");
                Self::load_ron(path).or_else(|| Self::load_checkpoint(path))
            }
        }
    }

    /// Loads a RON [Runtime] from the given path.
    #[cold]
    pub fn load_ron(path: &str) -> Option<Self> {
        log::info!(target: "Loading Runtime", "Trying to load RON...");

        match fs::read_to_string(path) {
            Ok(ser) => match ron::from_str::<Renderer>(&ser) {
                Ok(r) => return Some(Self::new(Arc::new(r), None)),
                Err(e) => {
                    log::error!(target: "Loading Runtime", "unable to deserialize RON: {}", e)
                }
            },
            Err(e) => log::error!(target: "Loading Runtime", "unable to read RON: {}", e),
        }

        None
    }

    /// Loads a checkpointed [Runtime] from the given path.
    /// The checkpoint may either be compressed (LZ4) or uncompressed.
    #[cold]
    pub fn load_checkpoint(path: &str) -> Option<Self> {
        log::info!(target: "Loading Runtime", "Trying to load checkpoint...");

        match fs::read(path) {
            Ok(ser) => return Some(Self::deserialize_checkpoint(&ser)),
            Err(e) => log::error!(target: "Loading Runtime", "unable to read checkpoint: {}", e),
        }

        None
    }

    pub fn run_frames(&self, frames: usize) {
        let progress = self.tile_progress.load(Ordering::SeqCst);
        let num_jobs = (frames * self.tiles).min(self.total_tiles - progress);
        log::trace!(target: "Runtime", "continuing from frame {} with {} jobs", progress / self.tiles, num_jobs);

        let threadpool = Threadpool::new(self.threadpool.workers(), None, None);
        let tiles = self.tiles;
        for _ in 0..num_jobs {
            let r = Arc::clone(&self.renderer);
            let p = self.tile_progress.clone();
            threadpool.execute(move || {
                let index = p.fetch_add(1, Ordering::SeqCst) % tiles;
                r.integrate(index);
            });
        }

        threadpool.join();
    }

    pub fn run(&self) {
        for _ in 0..self.threadpool.workers() {
            let c = self.cancel.clone();
            let r = self.renderer.clone();
            let p = self.tile_progress.clone();

            let total_tiles = self.total_tiles;
            let tiles = self.tiles;
            self.threadpool.execute(move || loop {
                if c.load(Ordering::Relaxed) {
                    break;
                }

                let tile_index = p.fetch_add(1, Ordering::SeqCst);
                if tile_index >= total_tiles {
                    break;
                }

                let index = tile_index % tiles;
                r.integrate(index);
            })
        }
    }

    pub fn done(&self) -> bool {
        self.tile_progress.load(Ordering::Relaxed) >= self.total_tiles
    }

    pub fn join_threadpool(&self) {
        self.threadpool.join();
    }

    pub fn save(&self) {
        let path = format!("{}.bin", &self.renderer.config.output);
        RuntimeSerde::from(self).save_to(path);
    }

    #[cold]
    pub fn deserialize_checkpoint(bytes: &[u8]) -> Self {
        let binary = decompress_size_prepended(bytes)
            .map_err(|e| log::error!(target: "Runtime", "unable to decompress checkpoint: {}", e))
            .unwrap();
        bincode::deserialize::<RuntimeSerde>(&binary)
            .map_err(|e| log::error!(target: "Runtime", "unable to deserialize checkpoint: {}", e))
            .unwrap()
            .into()
    }

    #[cold]
    pub fn serialize_checkpoint(&self) -> Vec<u8> {
        let binary = bincode::serialize(&RuntimeSerde::from(self))
            .map_err(|e| log::error!(target: "Runtime", "unable to serialize checkpoint: {}", e))
            .unwrap();
        compress_prepend_size(&binary)
    }
}

#[derive(Deserialize, Serialize)]
pub struct RuntimeSerde {
    renderer: Arc<Renderer>,
    tile_progress: Arc<AtomicUsize>,
}

impl RuntimeSerde {
    pub fn save_to<P: AsRef<Path>>(&self, path: P) {
        log::info!(target: "Runtime", "saving checkpoint...");
        let binary = bincode::serialize(self)
            .map_err(|e| log::error!(target: "Runtime", "unable to serialize checkpoint: {}", e))
            .unwrap();
        let compressed = compress_prepend_size(&binary);

        fs::write(path, compressed)
            .map_err(
                |e| log::error!(target: "Runtime", "unable to write checkpoint to file: {}", e),
            )
            .unwrap();
        log::info!(target: "Runtime", "saved checkpoint!");
    }
}

impl From<RuntimeSerde> for Runtime {
    fn from(r: RuntimeSerde) -> Self {
        Self::new(r.renderer, Some(r.tile_progress))
    }
}

impl From<Runtime> for RuntimeSerde {
    fn from(r: Runtime) -> Self {
        Self {
            renderer: r.renderer,
            tile_progress: r.tile_progress,
        }
    }
}

impl From<&Runtime> for RuntimeSerde {
    fn from(r: &Runtime) -> Self {
        Self {
            tile_progress: r.tile_progress.clone(),
            renderer: r.renderer.clone(),
        }
    }
}
