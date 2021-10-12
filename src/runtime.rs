use crate::renderer::Renderer;
use crate::util::threadpool::Threadpool;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use signal_hook::consts as signals;
use std::os::raw::c_int;
use std::sync::Arc;

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

    pub fn output_path(&self) -> &str {
        &self.renderer.config.output
    }

    fn watch(signal: c_int, watcher: Arc<AtomicBool>) -> Arc<AtomicBool> {
        let _ = signal_hook::flag::register(signal, Arc::clone(&watcher));
        watcher
    }

    fn create_bars(&self, tiles: usize, passes: usize) -> (ProgressBar, ProgressBar) {
        let tp_template = ProgressStyle::default_bar().template(
            "Render tiles:  {bar:40.cyan/white} {percent}% [{eta_precise} remaining]\n{msg}",
        );
        let fp_template = ProgressStyle::default_bar()
            .template("Render frames: {bar:40.cyan/white} {pos}/{len} {per_sec}");
        let bar = MultiProgress::new();
        let tp = bar.add(ProgressBar::new(tiles as u64));
        tp.set_style(tp_template);
        let fp = bar.add(ProgressBar::new(passes as u64));
        fp.set_style(fp_template);

        (tp, fp)
    }

    pub fn run(&self) -> (Threadpool, Arc<AtomicBool>, ProgressBar, ProgressBar) {
        let stop_watcher = Arc::new(AtomicBool::new(false));
        let stop_watcher = Self::watch(signals::SIGUSR1, stop_watcher);
        let stop_watcher = Self::watch(signals::SIGINT, stop_watcher);

        let threads = self.renderer.config.threads.unwrap_or_else(num_cpus::get);
        let c = Arc::clone(&stop_watcher);
        let threadpool = Threadpool::new(
            threads,
            None,
            Some(Box::new(move || c.store(true, Ordering::SeqCst))),
        );

        let frame_tiles = self.renderer.sensor().num_tiles();
        let total_tiles = frame_tiles * self.renderer.config.passes;

        let (tp, fp) = self.create_bars(total_tiles, self.renderer.config.passes);
        let checkpointed_progress = self.progress.load(Ordering::SeqCst);
        tp.inc(checkpointed_progress as u64);
        fp.inc((checkpointed_progress / frame_tiles) as u64);

        for _ in 0..threads {
            let c = Arc::clone(&stop_watcher);
            let r = Arc::clone(&self.renderer);
            let p = Arc::clone(&self.progress);

            let tp = tp.clone();
            let fp = fp.clone();

            threadpool.execute(move || loop {
                if c.load(Ordering::SeqCst) {
                    break;
                }

                let tile_index = p.fetch_add(1, Ordering::SeqCst);
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

        (threadpool, stop_watcher, tp, fp)
    }

    #[cfg(feature = "show-image")]
    pub fn run_live(
        &self,
    ) -> (
        Threadpool,
        Threadpool,
        Arc<AtomicBool>,
        ProgressBar,
        ProgressBar,
    ) {
        use show_image::create_window;

        let window = create_window("Rust-V2", Default::default()).unwrap();

        let (render_pool, cancelled, tp, fp) = self.run();

        let c = cancelled.clone();
        let r = self.renderer.clone();
        let termination = Arc::new(AtomicBool::new(false));
        let t = termination.clone();
        let image_pool = Threadpool::new(
            1,
            Some(1),
            Some(Box::new(move || t.store(true, Ordering::Relaxed))),
        );
        let tp_clone = tp.clone();
        image_pool.execute(move || {
            while termination.load(Ordering::Relaxed) && c.load(Ordering::Relaxed) {
                if let Err(e) = window.set_image("Rendering", r.get_image::<u8>()) {
                    tp_clone.set_message(e.to_string());
                    break;
                }
            }
        });

        (image_pool, render_pool, cancelled, tp, fp)
    }
}
