use rust_v2::renderer::Renderer;
use rust_v2::runtime::Runtime;
use std::env::args;
use std::fs;
use std::sync::atomic::Ordering;

fn main() {
    let scene_path = args().nth(1).unwrap();

    let renderer = deserialize_scene(&scene_path);
    let runtime = Runtime::new(renderer);

    let (pool, cancelled, tp, fp) = runtime.run();
    pool.join();

    match cancelled.load(Ordering::SeqCst) {
        true => {
            tp.abandon();
            fp.abandon();
            runtime.save_image()
        }
        false => {
            tp.finish();
            fp.finish();
            runtime.save_image()
        }
    }
}

fn deserialize_scene(path: &str) -> Renderer {
    let ser = fs::read_to_string(&path).unwrap();
    ron::from_str(&ser).unwrap()
}
