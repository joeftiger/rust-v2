use lz4_flex::decompress_size_prepended;
use std::fs;

fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let file = fs::read(&path).unwrap();
    let bytes = decompress_size_prepended(&file).unwrap();

    let ron = String::from_utf8(bytes).unwrap();
    fs::write(format!("{}.ron", path), ron).unwrap();
}
