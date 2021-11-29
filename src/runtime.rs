use crate::renderer::Renderer;
use crate::util::threadpool::Threadpool;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::fs;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use lz4_flex::decompress_size_prepended;

#[derive(Deserialize, Serialize)]
pub struct Runtime {
    pub renderer: Arc<Renderer>,
    pub progress: Arc<AtomicUsize>,
}

impl Runtime {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            renderer: Arc::new(renderer),
            progress: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Loads either a RON or checkpointed [Runtime] from the given path.
    pub fn load(path: &str) -> Option<Self> {
        match path.rsplit_once('.') {
            None => {
                log::warn!(target: "Loading Runtime", "Unknown file type, trying best-effort");
                Self::load_ron(path).or_else(|| Self::load_checkpoint(path))
            }
            Some((_, ".ron")) => Self::load_ron(path),
            Some((_, ".bin")) => Self::load_checkpoint(path),
            Some((_, ending)) => {
                log::warn!(target: "Loading Runtime", "Unknown file ending: {}, trying best-effort", ending);
                Self::load_ron(path).or_else(|| Self::load_checkpoint(path))
            }
        }
    }

    /// Loads a RON [Runtime] from the given path.
    pub fn load_ron(path: &str) -> Option<Self> {
        log::info!(target: "Loading Runtime", "Trying to load RON...");

        match fs::read_to_string(path) {
            Ok(ser) => match ron::from_str::<Renderer>(&ser) {
                Ok(r) => return Some(Self::new(r)),
                Err(e) => log::error!(target: "Loading Runtime", "unable to deserialize RON: {}", e),
            },
            Err(e) => log::error!(target: "Loading Runtime", "unable to read RON: {}", e),
        }

        None
    }

    /// Loads a checkpointed [Runtime] from the given path.
    /// The checkpoint may either be compressed (LZ4) or uncompressed.
    pub fn load_checkpoint(path: &str) -> Option<Self> {
        log::info!(target: "Loading Runtime", "Trying to load checkpoint...");

        match fs::read_to_string(path) {
            Ok(ser) => {
                let binary = match decompress_size_prepended(ser.as_bytes()) {
                    Ok(b) => b,
                    Err(_) => ser.into_bytes(),
                };

                match bincode::deserialize(&binary) {
                    Ok(r) => return Some(Self::new(r)),
                    Err(e) => log::error!(target: "Loading Runtime", "unable to deserialize checkpoint: {}", e),
                }
            }

            Err(e) => log::error!(target: "Loading Runtime", "unable to read checkpoint: {}", e),
        }

        None
    }

    pub fn output_path(&self) -> &str {
        &self.renderer.config.output
    }

    fn create_bars(&self, tiles: usize, passes: usize) -> (ProgressBar, ProgressBar) {
        let tp_template = ProgressStyle::default_bar().template(
            "Render tiles:  {bar:40.cyan/white} {percent}% [{eta_precise} remaining]\n{msg}",
        );
        let fp_template = ProgressStyle::default_bar()
            .template("Render frames: {pos}/{len} {per_sec}");
        let bar = MultiProgress::new();
        let tp = bar.add(ProgressBar::new(tiles as u64));
        tp.set_style(tp_template);
        let fp = bar.add(ProgressBar::new(passes as u64));
        fp.set_style(fp_template);

        (tp, fp)
    }

    pub fn create_pool(&self) -> (Threadpool, Arc<AtomicBool>) {
        let cancel = Arc::new(AtomicBool::new(false));

        let threads = self.renderer.config.threads.unwrap_or_else(num_cpus::get);
        let c = Arc::clone(&cancel);
        log::info!(target: "Runtime", "creating pool with {} threads", threads);
        let threadpool = Threadpool::new(
            threads,
            None,
            Some(Box::new(move || c.store(true, Ordering::SeqCst))),
        );

        (threadpool, cancel)
    }

    pub fn run_pool(&self, threadpool: &Threadpool, cancel: Arc<AtomicBool>, frames: usize) {
        let frame_tiles = self.renderer.sensor().num_tiles();

        let total_tiles = frame_tiles * frames;
        let current_progress = self.progress.load(Ordering::SeqCst);

        for _ in 0..threadpool.workers() {
            let c = Arc::clone(&cancel);
            let r = Arc::clone(&self.renderer);
            let p = Arc::clone(&self.progress);

            threadpool.execute(move || loop {
                if c.load(Ordering::SeqCst) {
                    break;
                }

                let tile_index = p.fetch_add(1, Ordering::SeqCst);
                if tile_index >=current_progress + frames || tile_index >= total_tiles {
                    break;
                }

                let index = tile_index % frame_tiles;
                r.integrate(index);
            })
        }
    }

    pub fn run(&self) -> (Threadpool, Arc<AtomicBool>, ProgressBar, ProgressBar) {
        log::info!(target: "Runtime", "setting up environment");
        let (threadpool, cancel) = self.create_pool();

        let frame_tiles = self.renderer.sensor().num_tiles();
        let total_tiles = frame_tiles * self.renderer.config.passes;

        let checkpointed_progress = self.progress.load(Ordering::SeqCst);
        log::info!(target: "Runtime", "starting/continuing at {}/{} tiles", checkpointed_progress, total_tiles);

        let (tp, fp) = self.create_bars(total_tiles, self.renderer.config.passes);
        tp.inc(checkpointed_progress as u64);
        fp.inc((checkpointed_progress / frame_tiles) as u64);

        log::info!(target: "Runtime", "starting {} render threads", threadpool.workers());
        for _ in 0..threadpool.workers() {
            let c = Arc::clone(&cancel);
            let r = Arc::clone(&self.renderer);
            let p = Arc::clone(&self.progress);

            let tp = tp.clone();
            let fp = fp.clone();

            threadpool.execute(move || loop {
                if c.load(Ordering::SeqCst) {
                    break;
                }

                let tile_index = p.fetch_add(1, Ordering::Relaxed);
                if tile_index >= total_tiles {
                    break;
                }

                let index = tile_index % frame_tiles;
                r.integrate(index);

                tp.inc(1);
                if index == frame_tiles - 1 {
                    fp.inc(1);
                }
            })
        }

        log::info!(target: "Runtime", "rendering in progress");
        (threadpool, cancel, tp, fp)
    }

    pub fn done(&self) -> bool {
        let total_tiles = self.renderer.sensor().num_tiles() * self.renderer.config.passes;

        self.progress.load(Ordering::Relaxed) >= total_tiles
    }

    /*#[cfg(feature = "show-image")]
    pub fn run_live(
        &self,
    ) -> (
        Threadpool,
        Arc<AtomicBool>,
        ProgressBar,
        ProgressBar,
    ) {
        use show_image::create_window;

        let (render_pool, cancel, tp, fp) = self.run();

        let c = Arc::clone(&cancel);
        let r = Arc::clone(&self.renderer);

        log::info!(target: "Runtime", "creating window");
        let window = create_window("Rust-V2", Default::default())
            .map_err(|e| log::error!(target: "Runtime", "unable to create window: {}", e)).unwrap();
        image_pool.execute(move || {
            while c.load(Ordering::Relaxed) {
                if let Err(e) = window.set_image("Rendering", r.get_image::<u8>()) {
                    log::error!(target: "Runtime", "unable to set image in window: {}", e);
                    break;
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        (render_pool, cancel, tp, fp)
    }*/
}
