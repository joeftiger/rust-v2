use core::fmt;
use core::str::FromStr;
use image::{io::Reader, ImageBuffer, Rgb};
use std::error::Error;
use std::fs;
use std::path::Path;

pub type Rgb16 = Rgb<u16>;
pub type Rgb16Image = ImageBuffer<Rgb16, Vec<u16>>;

static HELP: &str = r#"
USAGE:
    rust-v-stats <error_calc> <log_scale> <nth_pass> <target_image> <scenes>

EXAMPLE:
    rust-v-stats MSE true 5 cornell.png hero.ron random.ron high-depth.ron

ARGUMENTS:
    error_calc      {MAE, MBE, MSE, RMSE}   the algorithm to calculate the error
    log_scale       {true, false}           whether to use the natural logarithm to scale the MSE
    batch_size      INTEGER                 store every nth render pass into the history
    target_image    IMAGE                   the target image to calculate the MSE towards
    scenes          LIST OF SCENES          a list of scenes to include into the SVG

ALGORITHMS:
    MAE             Mean Absolute Error         sum of |x - y|
    MBE             Mean Bias Error             sum of (x - y)
    MSE             Mean Squared Error          sum of (x - y)²
    RMSE            Root Mean Squared Error     sqrt of sum of (x - y)²
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);

    let error_method = args.next().expect(HELP).parse().expect(HELP);
    let log_scale = args.next().expect(HELP).parse().expect(HELP);
    let batch_size = args.next().expect(HELP).parse().expect(HELP);

    let target_image_path = args.next().expect(HELP);
    let target_image = Reader::open(&target_image_path)?
        .with_guessed_format()?
        .decode()?
        .into_rgb16();

    let mut stats = Stats::new(target_image, error_method);
    let mut plot = poloto::plot(
        Path::new(&target_image_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        "passes/pixel",
        format!("{} (ln={})", error_method, log_scale),
    );
    plot.xmarker(0.0).ymarker(0.0);

    for scene_path in args {
        let content = fs::read_to_string(&scene_path).expect(HELP);
        let renderer = ron::from_str(&content).unwrap();
        let runtime = Runtime::new(renderer);

        let mse = stats.calculate(batch_size, runtime);

        let name = Path::new(&scene_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let plots: Vec<[f64; 2]> = mse
            .iter()
            .copied()
            .enumerate()
            .map(|x| {
                [
                    (x.0 * batch_size as usize) as f64,
                    if log_scale { x.1.ln() } else { x.1 },
                ]
            })
            .collect();

        plot.line(name, plots);
    }

    plot.simple_theme(poloto::upgrade_write(std::io::stdout()));

    Ok(())
}

use rust_v2::runtime::Runtime;
use ErrorMethod::*;

#[derive(Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum ErrorMethod {
    /// Mean Absolute Error
    MAE,
    /// Mean Bias Error
    MBE,
    /// Mean Squared Error
    MSE,
    /// Root Mean Squared Error
    RMSE,
}

impl ErrorMethod {
    pub fn calculate(&self, target: &[Rgb16], actual: &[Rgb16]) -> f64 {
        assert_eq!(target.len(), actual.len());

        let sum: f64 = target
            .iter()
            .zip(actual.iter())
            .map(|(left, right)| {
                let mut r = left[0] as f64 - right[0] as f64;
                let mut g = left[1] as f64 - right[1] as f64;
                let mut b = left[2] as f64 - right[2] as f64;

                r /= u16::MAX as f64;
                g /= u16::MAX as f64;
                b /= u16::MAX as f64;

                let (r, g, b) = match self {
                    MAE => (r.abs(), g.abs(), b.abs()),
                    MBE => (r, g, b),
                    RMSE | MSE => (r * r, g * g, b * b),
                };

                r + g + b
            })
            .sum();

        let intermediate = sum / (3.0 * target.len() as f64);

        match self {
            MAE | MBE | MSE => intermediate,
            RMSE => intermediate.sqrt(),
        }
    }
}

impl fmt::Display for ErrorMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MAE => write!(f, "MAE"),
            MBE => write!(f, "MBE"),
            MSE => write!(f, "MSE"),
            RMSE => write!(f, "RMSE"),
        }
    }
}

impl FromStr for ErrorMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MAE" => Ok(MAE),
            "MBE" => Ok(MBE),
            "MSE" => Ok(MSE),
            "RMSE" => Ok(RMSE),
            _ => Err(format!("Unknown Error Method: {}", s)),
        }
    }
}

struct Stats {
    target: Vec<Rgb16>,
    error_method: ErrorMethod,
}

impl Stats {
    pub fn new(target_image: Rgb16Image, error_method: ErrorMethod) -> Self {
        Self {
            target: target_image.pixels().copied().collect(),
            error_method,
        }
    }

    pub fn calculate(&mut self, batch_size: usize, runtime: Runtime) -> Vec<f64> {
        let mut error = Vec::new();

        let (pool, cancel) = runtime.create_pool();

        while !runtime.done() {
            runtime.run_pool(&pool, cancel.clone(), batch_size);

            let actual: Vec<Rgb16> = runtime.renderer.get_image().pixels().copied().collect();
            error.push(self.error_method.calculate(&self.target, &actual));
        }

        error
    }
}
