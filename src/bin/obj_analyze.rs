use cgmath::Bounded;
use rust_v2::geometry::obj::ObjFile;
use rust_v2::geometry::{max2, min2, Aabb};
use rust_v2::Vec3;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mesh = args()
        .nth(1)
        .ok_or_else(|| "No .obj file given".to_string())?;
    let obj = ObjFile::load(&mesh)?;

    let (min, max) = obj.vertices.iter().fold(
        (Vec3::max_value(), Vec3::min_value()),
        |(min, max), &next| (min2(min, next), max2(max, next)),
    );
    let aabb = Aabb::new(min, max);

    println!("Aabb:   {:#?}", aabb);
    println!("Center: {:#?}", aabb.center());
    println!("Size:   {:#?}", aabb.size());

    Ok(())
}
