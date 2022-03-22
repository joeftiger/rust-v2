#![feature(total_cmp)]
#![feature(int_roundings)]

use cgmath::{InnerSpace, Vector3};
use image::io::Reader;
use image::{ImageBuffer, Rgb};
use rust_v2::runtime::Runtime;
use serde::Serialize;
use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::fs;

type Rgb16 = Rgb<u16>;
type Rgb16Image = ImageBuffer<Rgb16, Vec<u16>>;

static HELP: &str = r#"
USAGE:
    rust_v2_error <target_img> <scenes>

EXAMPLE:
    rust_v2_error cornell.png hero.ron random.ron

ARGUMENTS:
    target_img      STRING              the target image path to calculate the difference towards
    scenes          LIST OF STRINGS     scene files (space separated) to load
"#;

use util::*;

fn main() {
    env_logger::init();
    start().unwrap_or_else(|e| panic!("{}\n{}", e, HELP));
}

fn start() -> Result<(), Box<dyn Error>> {
    let mut state = State::init_from_env()?;
    state.calc_errors();
    state.save()?;

    Ok(())
}

#[derive(Serialize)]
struct State {
    target_image_path: String,
    #[serde(skip)]
    target_image: Rgb16Image,
    #[serde(skip)]
    confs: Vec<String>,
    errors: HashMap<String, HashMap<ErrorCalc, Vec<f64>>>,
}

impl State {
    fn init_from_env() -> Result<Self, Box<dyn Error>> {
        log::info!(target: "Rust-V2-Error", "initializing...");

        let mut args = args().skip(1);

        let target_image_path = args.next().ok_or(HELP)?;
        let target_image = Reader::open(&target_image_path)?
            .with_guessed_format()?
            .decode()?
            .into_rgb16();

        let confs: Vec<String> = args.collect();
        let errors = HashMap::with_capacity(confs.len());

        Ok(Self {
            target_image_path,
            target_image,
            confs,
            errors,
        })
    }

    fn calc_errors(&mut self) {
        for conf in &self.confs {
            let runtime = Runtime::load(conf).unwrap();
            let output = &runtime.renderer.config.output;

            log::info!(target: "Rust-V2-Error", "calculating error for scene: {}", output);

            let mut errors = HashMap::with_capacity(ErrorCalc::num_types());
            for e in ErrorCalc::variants() {
                errors.insert(e, Vec::with_capacity(runtime.passes));
            }

            runtime.run_frames(1);
            for _ in 0..runtime.passes {
                let curr_image = runtime.renderer.get_image_png();
                runtime.run_frames(1);
                for e in &ErrorCalc::variants() {
                    let error = e.calc(&self.target_image, &curr_image);
                    errors.get_mut(e).unwrap().push(error);
                }
            }

            self.errors.insert(output.into(), errors);
        }
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = self.target_image_path.clone() + "-error.json";
        let contents = serde_json::to_string_pretty(self)?;

        match fs::write(path, contents) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[derive(Copy, Clone, Serialize, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
enum ErrorCalc {
    /// Mean Squared Error
    MSE,
    /// Peak Squared Error
    PSE,
    /// Peak Signal to Noise Ratio
    PSNR,
    /// Structural Similarity Index Measure
    SSIM,
    /// Variance
    VAR,
}

#[allow(dead_code)]
impl ErrorCalc {
    #[inline]
    pub const fn num_types() -> usize {
        5
    }

    #[inline]
    pub const fn variants() -> [Self; Self::num_types()] {
        [Self::MSE, Self::PSE, Self::PSNR, Self::SSIM, Self::VAR]
    }

    #[inline]
    fn calc(&self, original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        match self {
            Self::MSE => Self::mse(original, current),
            Self::PSE => Self::pse(original, current),
            Self::PSNR => Self::psnr(original, current),
            Self::SSIM => Self::ssim(original, current),
            Self::VAR => Self::var(original, current),
        }
    }

    fn mse(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        let n = original.pixels().len() as f64;

        se(original, current).iter().sum::<f64>() / n
    }

    fn pse(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        se(original, current)
            .into_iter()
            .max_by(f64::total_cmp)
            .unwrap()
    }

    fn psnr(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        // const max: f64 = u16::MAX as f64;
        // let A = 20.0 * f64::log10(Vec3::new(max, max, max))

        const A: f64 = 101.10067862250162;
        A - 10.0 * f64::log10(Self::mse(original, current))
    }

    fn ssim(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        let x = vec3s(original);
        let y = vec3s(current);

        let mu_x = average(&x);
        let mu_y = average(&y);

        let sigma2_x = variance2(&x, mu_x);
        let sigma2_y = variance2(&y, mu_y);
        let sigma_xy = covariance(&x, &y, mu_x, mu_y);

        /*
        const k1: f64 = 0.01;
        const k2: f64 = 0.03;
        const l: f64 = u16::MAX as f64;
        let c1 = (k1 * l).powi(2);
        let c2 = (k2 * l).powi(2);
         */
        const C1: f64 = 429483.6225;
        const C2: f64 = 3865352.6025;

        let top = (2.0 * mu_x * mu_y + C1) * (2.0 * sigma_xy + C2);
        let bot = (mu_x.powi(2) + mu_y.powi(2) + C1) * (sigma2_x + sigma2_y + C2);

        top / bot
    }

    fn var(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        let diff = diff(original, current);
        let mu = average(&diff);
        variance(&diff, mu)
    }
}

mod util {
    use super::*;

    #[inline(always)]
    pub fn px_to_vec3(px: &Rgb16) -> Vector3<f64> {
        Vector3::new(px[0] as f64, px[1] as f64, px[2] as f64)
    }

    pub fn vec3s(img: &Rgb16Image) -> Vec<Vector3<f64>> {
        img.pixels().map(px_to_vec3).collect()
    }

    pub fn diff(a: &Rgb16Image, b: &Rgb16Image) -> Vec<Vector3<f64>> {
        a.pixels()
            .map(px_to_vec3)
            .zip(b.pixels().map(px_to_vec3))
            .map(|(left, right)| left - right)
            .collect()
    }

    pub fn average(x: &[Vector3<f64>]) -> f64 {
        x.iter().map(|v| v.magnitude()).sum::<f64>() / x.len() as f64
    }

    pub fn variance(x: &[Vector3<f64>], mu: f64) -> f64 {
        let sum: f64 = x.iter().map(|v| (v.magnitude() - mu).powi(2)).sum();

        f64::sqrt(sum / (x.len() - 1) as f64)
    }

    pub fn variance2(x: &[Vector3<f64>], mu: f64) -> f64 {
        let sum: f64 = x.iter().map(|v| v.magnitude2()).sum();
        sum / x.len() as f64 - mu.powi(2)
    }

    pub fn covariance(x: &[Vector3<f64>], y: &[Vector3<f64>], avg_x: f64, avg_y: f64) -> f64 {
        x.iter()
            .map(|v| v.magnitude())
            .zip(y.iter().map(|v| v.magnitude()))
            .map(|(a, b)| (a - avg_x) * (b - avg_y))
            .sum::<f64>()
            / (x.len() - 1) as f64
    }

    /// Calculates the Square Error
    pub fn se(original: &Rgb16Image, current: &Rgb16Image) -> Vec<f64> {
        diff(original, current)
            .iter()
            .map(|v| v.magnitude2())
            .collect()
    }
}
