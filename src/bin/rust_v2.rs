use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rust_v2::runtime::Runtime;
use signal_hook::consts as signals;
use std::env::args;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

// #[cfg(not(feature = "show-image"))]
fn main() {
    env_logger::init();
    run()
}

// #[show_image::main]
// #[cfg(feature = "show-image")]
// fn main() {
//     run();
//     show_image::exit(0);
// }

fn run() {
    let scene_path = args().nth(1).unwrap();
    let runtime = Arc::new(deserialize_runtime(&scene_path));

    // #[cfg(not(feature = "show-image"))]
    let (render_pool, cancel, tp, fp) = runtime.run();
    // #[cfg(feature = "show-image")]
    // let (render_pool, cancel, tp, fp) = runtime.run_live();

    watch_signals(Arc::clone(&cancel));
    watch_save_image_signal(Arc::clone(&cancel), Arc::clone(&runtime));
    watch_save_checkpoint_signal(Arc::clone(&cancel), Arc::clone(&runtime));

    render_pool.join();
    // #[cfg(feature = "show-image")]
    // image_pool.terminate();

    match cancel.load(Ordering::SeqCst) {
        true => {
            tp.abandon();
            fp.abandon();
            save_image(&runtime);
            save_progress(&runtime);
        }
        false => {
            tp.finish();
            fp.finish();
            save_image(&runtime);
        }
    }
}

fn watch_signals(cancel: Arc<AtomicBool>) {
    let _ = signal_hook::flag::register(signals::SIGINT, Arc::clone(&cancel)).map_err(
        |e| log::warn!(target: "Runtime", "unable to create signal watcher for SIGINT: {}", e),
    );
    let _ = signal_hook::flag::register(signals::SIGTERM, cancel).map_err(
        |e| log::warn!(target: "Runtime", "unable to create signal watcher for SIGTERM: {}", e),
    );
}

fn watch_save_image_signal(cancel: Arc<AtomicBool>, runtime: Arc<Runtime>) {
    let save_flag = Arc::new(AtomicBool::new(false));
    match signal_hook::flag::register(signals::SIGUSR1, Arc::clone(&save_flag)) {
        Ok(_) => {
            std::thread::spawn(move || {
                while !cancel.load(Ordering::Relaxed) {
                    if save_flag.load(Ordering::Relaxed) {
                        save_flag.store(false, Ordering::Relaxed);

                        save_image(&runtime);
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });
            log::info!(target: "Runtime", "registered SIGUSR1 for saving rendering");
        }
        Err(e) => {
            log::warn!(target: "Runtime", "unable to register SIGUSR1 for saving rendering: {}", e);
        }
    }
}

fn watch_save_checkpoint_signal(cancel: Arc<AtomicBool>, runtime: Arc<Runtime>) {
    let save_flag = Arc::new(AtomicBool::new(false));
    match signal_hook::flag::register(signals::SIGUSR2, Arc::clone(&save_flag)) {
        Ok(_) => {
            std::thread::spawn(move || {
                while !cancel.load(Ordering::Relaxed) {
                    if save_flag.load(Ordering::Relaxed) {
                        save_flag.store(false, Ordering::Relaxed);

                        save_progress(&runtime);
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            });
            log::info!(target: "Runtime", "registered SIGUSR2 for checkpointing");
        }
        Err(e) => {
            log::warn!(target: "Runtime", "unable to register SIGUSR2 for checkpointing: {}", e);
        }
    }
}

fn deserialize_runtime(path: &str) -> Runtime {
    if path.ends_with(".bin") {
        log::info!(target: "Runtime", "loading checkpoint: {}", path);
        let ser = fs::read(path)
            .map_err(|e| log::error!(target: "Runtime", "unable to load checkpoint: {}", e))
            .unwrap();
        let binary = decompress_size_prepended(&ser)
            .map_err(|e| log::error!(target: "Runtime", "unable to decompress checkpoint: {}", e))
            .unwrap();
        bincode::deserialize(&binary)
            .map_err(|e| log::error!(target: "Runtime", "unable to deserialize checkpoint: {}", e))
            .unwrap()
    } else {
        log::info!(target: "Runtime", "loading config: {}", path);
        let ser = fs::read_to_string(path)
            .map_err(|e| log::error!(target: "Runtime", "unable to load checkpoint: {}", e))
            .unwrap();
        let renderer = ron::from_str(&ser)
            .map_err(|e| log::error!(target: "Runtime", "unable to deserialize checkpoint: {}", e))
            .unwrap();
        Runtime::new(renderer)
    }
}

fn save_progress(runtime: &Runtime) {
    log::info!(target: "Runtime", "saving checkpoint...");

    let binary = bincode::serialize(&runtime)
        .map_err(|e| log::error!(target: "Runtime", "unable to serialize checkpoint: {}", e))
        .unwrap();
    debug_assert!(bincode::deserialize::<Runtime>(&binary).is_ok());

    let compressed = compress_prepend_size(&binary);
    debug_assert_eq!(&binary, &decompress_size_prepended(&compressed).unwrap());

    let path = runtime.output_path().to_string() + ".bin";
    fs::write(path, &compressed)
        .map_err(|e| log::error!(target: "Runtime", "unable to save checkpoint: {}", e))
        .unwrap();

    log::info!(target: "Runtime", "saved checkpoint!");
}

fn save_image(runtime: &Runtime) {
    log::info!(target: "Runtime", "saving image...");

    runtime
        .renderer
        .get_image::<u16>()
        .save(runtime.output_path().to_string() + ".png")
        .map_err(|e| log::error!(target: "Runtime", "unable to save image: {}", e))
        .unwrap();

    log::info!(target: "Runtime", "saved image!");
}
