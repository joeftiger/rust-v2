use rust_v2::runtime::Runtime;
use std::env::args;
use std::sync::atomic::Ordering;

fn main() {
    env_logger::init();
    start()
}

fn start() {
    let scene_path = args().nth(1).unwrap();
    let runtime = Runtime::load(&scene_path).unwrap();
    runtime.run();
    runtime.join_threadpool();

    runtime.renderer.save_image();

    if runtime.cancel.load(Ordering::Relaxed) {
        runtime.save();
    }
}
