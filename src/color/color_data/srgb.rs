#![allow(clippy::excessive_precision)]
use crate::Float;

// data taken from: https://en.wikipedia.org/wiki/CIE_1931_color_space#CIE_RGB_color_space
pub const LAMBDAS: [Float; 3] = [0.4358, 0.5461, 0.7];

// NOTE THE INVERSION OF VALUES!
// This is because our colour classes rank values by wavelength which is effectively "inverted".

#[rustfmt::skip]
pub const DARK_SKIN:     [Float; 3] = [0.265433396324784, 0.320827324940405, 0.451985507348026];
#[rustfmt::skip]
pub const LIGHT_SKIN:    [Float; 3] = [0.503055950907360, 0.582738716853994, 0.765972935723141];
#[rustfmt::skip]
pub const BLUE_SKY:      [Float; 3] = [0.614439013487211, 0.481216296689985, 0.365221128960770];
#[rustfmt::skip]
pub const FOLIAGE:       [Float; 3] = [0.253370936701835, 0.424161374740617, 0.355649637522838];
#[rustfmt::skip]
pub const BLUE_FLOWER:   [Float; 3] = [0.687225606075082, 0.504123153746312, 0.510656240093606];
#[rustfmt::skip]
pub const BLUISH_GREEN:  [Float; 3] = [0.668344206176023, 0.747829256672327, 0.386089655534932];
#[rustfmt::skip]
pub const ORANGE:        [Float; 3] = [0.179077843041195, 0.483523828968116, 0.863093418722002];
#[rustfmt::skip]
pub const PURPLISH_BLUE: [Float; 3] = [0.659181811904566, 0.359504879519143, 0.282440643630781];
#[rustfmt::skip]
pub const MODERATE_RED:  [Float; 3] = [0.381460259554855, 0.328914914848483, 0.762071779791923];
#[rustfmt::skip]
pub const PURPLE:        [Float; 3] = [0.409221031391268, 0.232274686016055, 0.355981329046635];
#[rustfmt::skip]
pub const YELLOW_GREEN:  [Float; 3] = [0.243107906845461, 0.740552451504050, 0.629798142147472];
#[rustfmt::skip]
pub const ORANGE_YELLOW: [Float; 3] = [0.158306058293684, 0.629593762645567, 0.896249844549554];
#[rustfmt::skip]
pub const BLUE:          [Float; 3] = [0.576195209901642, 0.245483591390485, 0.165488593203341];
#[rustfmt::skip]
pub const GREEN:         [Float; 3] = [0.281234622466667, 0.585394786073615, 0.283264799647560];
#[rustfmt::skip]
pub const RED:           [Float; 3] = [0.221885082963718, 0.195538129501934, 0.686533689604943];
#[rustfmt::skip]
pub const YELLOW:        [Float; 3] = [0.083741974909830, 0.782979200607304, 0.934061366943582];
#[rustfmt::skip]
pub const MAGENTA:       [Float; 3] = [0.588239890055834, 0.329748012747315, 0.737111726481978];
#[rustfmt::skip]
pub const CYAN:          [Float; 3] = [0.652102190441094, 0.536010099061219, -0.185440641648003];
#[rustfmt::skip]
pub const WHITE:         [Float; 3] = [0.940214035518687, 0.962026354255479, 0.962070206926636];
#[rustfmt::skip]
pub const GREY_1:        [Float; 3] = [0.787707064909730, 0.792583731311461, 0.786738853998503];
#[rustfmt::skip]
pub const GREY_2:        [Float; 3] = [0.633147395616370, 0.635143395725413, 0.630287624455572];
#[rustfmt::skip]
pub const GREY_3:        [Float; 3] = [0.474854663555374, 0.475953628435172, 0.470151683602212];
#[rustfmt::skip]
pub const GREY_4:        [Float; 3] = [0.333384558671776, 0.331779293144700, 0.326433680189283];
#[rustfmt::skip]
pub const BLACK:         [Float; 3] = [0.198420076807872, 0.196313606429862, 0.195684876663901];
