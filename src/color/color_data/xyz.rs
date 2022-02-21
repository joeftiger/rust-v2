#![allow(clippy::excessive_precision)]
use crate::Float;

// data taken from: https://en.wikipedia.org/wiki/CIE_1931_color_space#CIE_RGB_color_space
pub const LAMBDAS: [Float; 3] = [0.4358, 0.5461, 0.7];

// NOTE THE INVERSION OF VALUES!
// This is because our colour classes rank values by wavelength which is effectively "inverted".

#[rustfmt::skip]
pub const DARK_SKIN:     [Float; 3] = [0.344590937501314, 0.188506158360751, 0.176312929913317];
#[rustfmt::skip]
pub const LIGHT_SKIN:    [Float; 3] = [0.252303659791320, 0.345811066597506, 0.371889832835184];
#[rustfmt::skip]
pub const BLUE_SKY:      [Float; 3] = [0.344590937501314, 0.188506158360751, 0.176312929913317];
#[rustfmt::skip]
pub const FOLIAGE:       [Float; 3] = [0.069579226320272, 0.133446563636559, 0.106062833410069];
#[rustfmt::skip]
pub const BLUE_FLOWER:   [Float; 3] = [0.438942686356375, 0.234498189027205, 0.247903577298786];
#[rustfmt::skip]
pub const BLUISH_GREEN:  [Float; 3] = [0.448398224503906, 0.426671792142612, 0.309438520596337];
#[rustfmt::skip]
pub const ORANGE:        [Float; 3] = [0.063198287096588, 0.296684423138667, 0.371521058750907];
#[rustfmt::skip]
pub const PURPLISH_BLUE: [Float; 3] = [0.386477306667956, 0.118060071047698, 0.135471583051176];
#[rustfmt::skip]
pub const MODERATE_RED:  [Float; 3] = [0.135271915603418, 0.187051997571252, 0.276654809519086];
#[rustfmt::skip]
pub const PURPLE:        [Float; 3] = [0.139758645533926, 0.063712013282346, 0.083842636054098];
#[rustfmt::skip]
pub const YELLOW_GREEN:  [Float; 3] = [0.113160364986810, 0.442099401794478, 0.336503218473883];
#[rustfmt::skip]
pub const ORANGE_YELLOW: [Float; 3] = [0.077792765526787, 0.420741866523283, 0.452261410872749];
#[rustfmt::skip]
pub const BLUE:          [Float; 3] = [0.283282955677197, 0.061107278771625, 0.079775120066970];
#[rustfmt::skip]
pub const GREEN:         [Float; 3] = [0.098323013697585, 0.234319374045395, 0.146406294508022];
#[rustfmt::skip]
pub const RED:           [Float; 3] = [0.050410614417318, 0.116851624936967, 0.195589470375960];
#[rustfmt::skip]
pub const YELLOW:        [Float; 3] = [0.092440702801338, 0.594193224248407, 0.560398850505099];
#[rustfmt::skip]
pub const MAGENTA:       [Float; 3] = [0.310143435230853, 0.192452166367174, 0.294128596615950];
#[rustfmt::skip]
pub const CYAN:          [Float; 3] = [0.392870020314913, 0.199513415753917, 0.146210811588072];
#[rustfmt::skip]
pub const WHITE:         [Float; 3] = [0.953006162336230, 0.912449408084849, 0.862082817473392];
#[rustfmt::skip]
pub const GREY_1:        [Float; 3] = [0.635944680278189, 0.588675045348572, 0.556560035411869];
#[rustfmt::skip]
pub const GREY_2:        [Float; 3] = [0.390701036689185, 0.359646461343564, 0.340277223852805];
#[rustfmt::skip]
pub const GREY_3:        [Float; 3] = [0.208547091820333, 0.191321198763686, 0.180684867043266];
#[rustfmt::skip]
pub const GREY_4:        [Float; 3] = [0.098760848931306, 0.089407984961901, 0.084458522820453];
#[rustfmt::skip]
pub const BLACK:         [Float; 3] = [0.035418484065876, 0.031973719431692, 0.030422104614276];
