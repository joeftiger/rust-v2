use rust_v2::runtime::Runtime;

static TEST: &str = include_str!("../../res/scenes/cornell.ron");

fn main() {
    let renderer = ron::from_str(TEST).unwrap();
    let runtime = Runtime::new(renderer);

    let binary = bincode::serialize(&runtime).unwrap();
    let c_lz4 = lz4_flex::compress_prepend_size(&binary);
    println!("raw size: {:>9} B", binary.len());
    println!("lz4 size: {:>9} B", c_lz4.len());
    let d_lz4 = lz4_flex::decompress_size_prepended(&c_lz4).unwrap();
    let _ = bincode::deserialize::<Runtime>(&d_lz4).unwrap();
}
