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

    fn create_bars(tiles: usize, passes: usize) -> (ProgressBar, ProgressBar) {
        let tp_template = ProgressStyle::default_bar()
            .template("Render tiles:  {bar:40.cyan/white} {percent}% [{eta_precise} remaining]");
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
        let stop_watcher = Self::watch(signals::SIGTERM, Arc::new(AtomicBool::new(false)));
        let stop_watcher = Self::watch(signals::SIGINT, stop_watcher);

        let threads = self.renderer.config.threads.unwrap_or_else(num_cpus::get);
        let c = Arc::clone(&stop_watcher);
        let threadpool = Threadpool::new(
            threads,
            None,
            Some(Box::new(move || c.store(true, Ordering::SeqCst))),
        );

        let fram_tiles = self.renderer.sensor().num_tiles();
        let total_tiles = fram_tiles * self.renderer.config.passes;

        let (tp, fp) = Self::create_bars(total_tiles, self.renderer.config.passes);

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

                let index = tile_index % fram_tiles;
                r.integrate(index);

                tp.inc(1);
                if index == fram_tiles - 1 {
                    fp.inc(1);
                }
            })
        }

        (threadpool, stop_watcher, tp, fp)
    }

    #[cfg(feature = "show-image")]
    pub fn run_live(&self) -> (Threadpool, Arc<AtomicBool>, ProgressBar, ProgressBar) {
        use core::time::Duration;
        use show_image::{create_window, event};

        let window = create_window("Rust-V2", Default::default()).unwrap();

        let (threadpool, cancelled, tp, fp) = self.run();

        'main: loop {
            if let Ok(e) = window
                .event_channel()
                .unwrap()
                .recv_timeout(Duration::from_secs(1))
            {
                if let event::WindowEvent::KeyboardInput(event) = e {
                    if event.input.state.is_pressed() {
                        if let Some(key) = event.input.key_code {
                            match key {
                                event::VirtualKeyCode::Escape => {
                                    cancelled.store(true, Ordering::SeqCst);
                                    break 'main;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            if let Err(err) = window.set_image("Rendering", self.renderer.get_image::<u8>()) {
                eprintln!("{}\nSkipping this image!", err);
            }
        }

        (threadpool, cancelled, tp, fp)
    }
}
