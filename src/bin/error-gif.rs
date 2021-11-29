#![feature(int_abs_diff)]

use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::ops::Range;

use image::{ImageBuffer, io::Reader, Rgb};
use itertools::{Itertools, MinMaxResult};
use plotters::prelude::*;

use rust_v2::runtime::Runtime;

pub type Rgb16 = Rgb<u16>;
pub type Rgb16Image = ImageBuffer<Rgb16, Vec<u16>>;

static HELP: &str = r#"
USAGE:
    error-gif <target_img> <frame_steps> <scene> <out>

EXAMPLE:
    error-gif cornell.png 10 hero.ron out.gif

ARGUMENTS:
    target_img      STRING       the target image path to calculate the difference towards
    frame_steps     INTEGER      store every these-steps into the history
    scene           STRING       the scene file to load
    out             STRING       output path of gif
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);

    let target_image_path = args.next().expect(HELP);
    let target_image = Reader::open(&target_image_path)?
        .with_guessed_format()?
        .decode()?
        .into_rgb16();
    let frame_steps = args.next().expect(HELP).parse().expect(HELP);
    let runtime = Runtime::load(&args.next().expect(HELP)).expect(HELP);

    let data = ChartData::new(runtime, frame_steps, target_image);

    let root = BitMapBackend::gif(args.next().expect(HELP), (800, 800), 100)?.into_drawing_area();

    for pitch in 0..157 {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("2D Guassian PDF", ("sans-serif", 20))
            .build_cartesian_3d(data.deviation.clone(), data.values.clone(), data.time_range.clone())?;
        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart.configure_axes().draw()?;

        chart.draw_series(
            SurfaceSeries::xoz(data.deviation.clone(), data.time_range.clone(), |x, z| data.f(x, z))
                .style_func(&|&v| {
                    (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v as f64 / 5.0, 1.0, 0.7)).into()
                }),
        )?;

        root.present()?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file");

    Ok(())
}

struct ChartData {
    time: Vec<HashMap<u32, u32>>,
    deviation: Range<u32>,
    values: Range<u32>,
    time_range: Range<u32>,
}

impl ChartData {
    pub fn new(runtime: Runtime, frame_steps: usize, target_img: Rgb16Image) -> Self {
        let mut time = Vec::with_capacity((runtime.renderer.config.passes as f64 / frame_steps as f64).ceil() as usize);

        let (pool, c) = runtime.create_pool();
        while !runtime.done() {
            let mut diffs = HashMap::new();

            runtime.run_pool(&pool, c.clone(), frame_steps);
            let img: Rgb16Image = runtime.renderer.get_image();
            target_img.pixels().zip(img.pixels()).for_each(|(target, actual)| {
                let r = u16::abs_diff(target[0], actual[0]) as u32;
                let g = u16::abs_diff(target[1], actual[1]) as u32;
                let b = u16::abs_diff(target[2], actual[2]) as u32;

                *diffs.entry(r).or_insert(0) += 1;
                *diffs.entry(g).or_insert(0) += 1;
                *diffs.entry(b).or_insert(0) += 1;
            });

            time.push(diffs);
        }

        let (x_min, x_max) = match time.iter().flat_map(|m| m.iter()).minmax_by(|l, r| l.0.cmp(r.0)) {
            MinMaxResult::NoElements => unreachable!(),
            MinMaxResult::OneElement(_) => unreachable!(),
            MinMaxResult::MinMax((&min, _), (&max, _)) => (min, max),
        };
        let (z_min, z_max) = match time.iter().flat_map(|m| m.iter()).minmax_by(|l, r| l.1.cmp(r.1)) {
            MinMaxResult::NoElements => unreachable!(),
            MinMaxResult::OneElement(_) => unreachable!(),
            MinMaxResult::MinMax((_, &min), (_, &max)) => (min, max),
        };

        let deviation = x_min..x_max;
        let values = z_min..z_max;
        let time_range = 0..time.len() as u32;

        Self {
            time,
            deviation,
            values,
            time_range
        }
    }

    pub fn f(&self, x: u32, z: u32) -> u32 {
        let map = &self.time[z as usize];
        map.get(&x).copied().unwrap_or(0)
    }
}