#![feature(int_abs_diff)]
#![feature(array_zip)]

use std::env::args;
use std::error::Error;
use image::io::Reader;
use image::{ImageBuffer, Pixel, Rgb};
use itertools::{Itertools, MinMaxResult};

pub type Rgb16 = Rgb<u16>;
pub type Rgb16Image = ImageBuffer<Rgb16, Vec<u16>>;

static HELP: &str = r#"
USAGE:
    img-diff <IMG1> <IMG2> <output>

EXAMPLE:
    img-diff a.png b.png diff.png

ARGUMENTS:
    IMG1        path of image
    IMG2        path of image
    output      path of output difference image
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);
    let a = args.next().expect(HELP);
    let b = args.next().expect(HELP);
    let output = args.next().expect(HELP);

    let mut a = Reader::open(&a)?
        .with_guessed_format()?
        .decode()?
        .into_rgb16();
    let b = Reader::open(&b)?
        .with_guessed_format()?
        .decode()?
        .into_rgb16();

    let mut diffs = Vec::with_capacity(3 * (a.width() * a.height()) as usize);

    for x in 0..a.width() {
        for y in 0..b.width() {
            let a_px = a.get_pixel_mut(x, y);
            let b_px = b.get_pixel(x, y);
            a_px.0.iter().zip(b_px.0.iter()).map(|(&x, &y)| x as f64 / y as f64).for_each(|d| diffs.push(d));

            a_px.apply2(b_px, |i, j| i.abs_diff(j));
        }
    }

    let (min, max) = match diffs.iter().minmax() {
        MinMaxResult::NoElements => (0.0, 0.0),
        MinMaxResult::OneElement(&e) => (e, e),
        MinMaxResult::MinMax(&min, &max) => (min, max),
    };
    let mean_diff = diffs.iter().sum::<f64>() / diffs.len() as f64;
    println!("Min:     \t{}\nMean diff:\t{}\nMax:     \t{}", min, mean_diff, max);

    a.save(&output).unwrap();

    Ok(())
}