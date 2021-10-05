use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use rust_v2::runtime::Runtime;
use std::env::args;
use std::sync::atomic::Ordering;
use std::{fs, thread};

#[cfg(not(feature = "show-image"))]
fn main() {
    run()
}

#[show_image::main]
#[cfg(feature = "show-image")]
fn main() {
    run();
    show_image::exit(0);
}

fn run() {
    let scene_path = args().nth(1).unwrap();
    let runtime = deserialize_runtime(&scene_path);

    #[cfg(not(feature = "show-image"))]
    let (pool, cancelled, tp, fp) = runtime.run();
    #[cfg(feature = "show-image")]
    let (pool, cancelled, tp, fp) = runtime.run_live();

    pool.join();

    match cancelled.load(Ordering::SeqCst) {
        true => {
            tp.abandon();
            fp.abandon();
            save_image(&runtime);
            save_progress(runtime);
        }
        false => {
            tp.finish();
            fp.finish();
            save_image(&runtime);
        }
    }
}

fn deserialize_runtime(path: &str) -> Runtime {
    if path.ends_with(".bin") {
        let ser = fs::read(&path).unwrap();
        let binary = decompress_size_prepended(&ser).unwrap();
        bincode::deserialize(&binary).unwrap()
    } else {
        let ser = fs::read_to_string(&path).unwrap();
        let renderer = ron::from_str(&ser).unwrap();
        Runtime::new(renderer)
    }
}

fn save_progress(runtime: Runtime) {
    thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let binary = bincode::serialize(&runtime).unwrap();
            let compressed = compress_prepend_size(&binary);
            let path = runtime.output_path().to_string() + ".bin";
            fs::write(path, &compressed).unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
}

fn save_image(runtime: &Runtime) {
    runtime
        .renderer
        .get_image::<u16>()
        .save(runtime.output_path().to_string() + ".png")
        .unwrap()
}
