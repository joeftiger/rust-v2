#![feature(total_cmp)]

use cgmath::{InnerSpace, Vector3};
use image::io::Reader;
use image::{ImageBuffer, Rgb};
use rust_v2::runtime::Runtime;
use std::env::args;
use std::error::Error;

pub type Rgb16 = Rgb<u16>;
pub type Rgb16Image = ImageBuffer<Rgb16, Vec<u16>>;

static HELP: &str = r#"
USAGE:
    error-gif <log_scale> <frame_steps> <target_img> <scene0> <scene1> <out>

EXAMPLE:
    error-gif false 10 cornell.png hero.ron out.gif

ARGUMENTS:
    log_scale       BOLEAN      whether to use the log scale
    frame_steps     INTEGER     store every these-steps into the history
    target_img      STRING      the target image path to calculate the difference towards
    scene           STRING      the scene file to load
    out             STRING      output path of gif
"#;

use util::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);

    let log_scale: bool = args.next().expect(HELP).parse().expect(HELP);
    let frame_steps: usize = args.next().expect(HELP).parse().expect(HELP);
    let target_image_path = args.next().expect(HELP);
    let original = Reader::open(&target_image_path)?
        .with_guessed_format()?
        .decode()?
        .into_rgb16();
    let scene0_name = args.next().expect(HELP);
    let scene1_name = args.next().expect(HELP);
    let scene0 = Runtime::load(&scene0_name).expect(HELP);
    let scene1 = Runtime::load(&scene1_name).expect(HELP);

    let mut plots = ErrorType::variants().map(|e| {
        poloto::plot(
            e.to_string(),
            "passes/pixel",
            format!("{} (ln={})", e.to_string(), log_scale),
        )
    });

    let mut errors = [Vec::new(); ErrorType::num_types()];
    let (pool0, cancel) = scene0.create_pool();
    let (pool1, _) = scene1.create_pool();
    while !scene1.done() {
        scene1.run_pool(&pool, cancel.clone(), frame_steps);
        let current = scene1.renderer.get_image();

        for i in 0..ErrorType::num_types() {
            let e = ErrorType::variant(i);
            let error = e.calc(&original, &current);

            errors[i].push(error);
        }
    }

    for i in 0..ErrorType::num_types() {
        plots[i].line()
    }

    Ok(())
}

#[derive(Copy, Clone)]
enum ErrorType {
    /// Mean Squared Error
    MSE,
    /// Peak Squared Error
    PSE,
    /// Signal to Noise Ratio
    SNR,
    /// Peak Signal to Noise Ratio
    PSNR,
    /// Structural Similarity Index Measure
    SSIM,
}

impl ErrorType {
    pub const fn num_types() -> usize {
        5
    }

    pub const fn variant(i: usize) -> Self {
        match i {
            0 => Self::MSE,
            1 => Self::PSE,
            2 => Self::SNR,
            3 => Self::PSNR,
            4 => Self::SSIM,
            _ => panic!(),
        }
    }

    pub const fn variants() -> [Self; Self::num_types()] {
        [Self::MSE, Self::PSE, Self::SNR, Self::PSNR, Self::SSIM]
    }

    fn calc(&self, original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        match self {
            ErrorType::MSE => Self::mse(original, current),
            ErrorType::PSE => Self::pse(original, current),
            ErrorType::SNR => Self::snr(original, current),
            ErrorType::PSNR => Self::psnr(original, current),
            ErrorType::SSIM => Self::ssim(original, current),
        }
    }

    fn mse(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        let n = original.pixels().len() as f64;

        se(original, current).iter().sum::<f64>() / n
    }

    fn pse(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        se(original, current).into_iter().max_by(f64::total_cmp).unwrap()
    }

    fn snr(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        let signal = vec3s(current);
        let mu_signal = average(&signal);
        let sigma2_signal = variance2(&signal, mu_signal);

        let noise = diff(original, current);
        let mu_noise = average(&noise);
        let sigma2_noise = variance2(&noise, mu_noise);

        f64::sqrt(sigma2_signal / sigma2_noise - 1.0)
    }

    fn psnr(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        20.0 * f64::log10(u16::MAX as f64) - 10.0 * f64::log10(Self::mse(original, current))
    }

    fn ssim(original: &Rgb16Image, current: &Rgb16Image) -> f64 {
        let x = vec3s(original);
        let y = vec3s(current);

        let mu_x = average(&x);
        let mu_y = average(&y);
        let sigma2_x = variance2(&x, mu_x);
        let sigma2_y = variance2(&y, mu_y);
        let sigma_xy = covariance(&x, &y, mu_x, mu_y);

        let k1 = 0.01;
        let k2 = 0.03;
        let l = u16::MAX as f64;
        let c1 = (k1 * l).powi(2);
        let c2 = (k2 * l).powi(2);

        (2.0 * mu_x * mu_y + c1) * (2.0 * sigma_xy + c2)
            / ((mu_x.powi(2) + mu_y.powi(2) + c1) * (sigma2_x + sigma2_y + c2))
    }
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            ErrorType::MSE => "MSE".into(),
            ErrorType::PSE => "PSE".into(),
            ErrorType::SNR => "SNR".into(),
            ErrorType::PSNR => "PSNR".into(),
            ErrorType::SSIM => "SSIM".into(),
        }
    }
}

mod util {
    use super::*;

    pub fn px_to_vec3(px: &Rgb16) -> Vector3<f64> {
        Vector3::new(px[0] as f64, px[1] as f64, px[2] as f64)
    }

    pub fn vec3s(img: &Rgb16Image) -> Vec<Vector3<f64>> {
        img.pixels().map(|px| px_to_vec3(px)).collect()
    }

    pub fn diff(a: &Rgb16Image, b: &Rgb16Image) -> Vec<Vector3<f64>> {
        a.pixels()
            .map(|px| px_to_vec3(px))
            .zip(b.pixels().map(|px| px_to_vec3(px)))
            .map(|(left, right)| left - right)
            .collect()
    }

    pub fn average(x: &[Vector3<f64>]) -> f64 {
        x.iter().map(|v| v.magnitude()).sum::<f64>() / x.len() as f64
    }

    pub fn variance2(x: &[Vector3<f64>], mu: f64) -> f64 {
        let sum: f64 = x.iter().map(|v| (v.magnitude() - mu).powi(2)).sum();
        sum / x.len() as f64
    }

    pub fn covariance(x: &[Vector3<f64>], y: &[Vector3<f64>], avg_x: f64, avg_y: f64) -> f64 {
        x.iter()
            .map(|v| v.magnitude())
            .zip(y.iter().map(|v| v.magnitude()))
            .map(|(a, b)| (a - avg_x) * (b - avg_y))
            .sum::<f64>()
            / x.len() as f64
    }

    /// Calculates the Square Error
    pub fn se(original: &Rgb16Image, current: &Rgb16Image) -> Vec<f64> {
        diff(original, current)
            .iter()
            .map(|v| v.magnitude2())
            .collect()
    }
}
