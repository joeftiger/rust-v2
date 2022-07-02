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

// Babel Srgb u16 / 65535
// #[rustfmt::skip]
// pub const DARK_SKIN:     [Float; 3] = [0.2656137941557946, 0.3207751583123522, 0.452124818799115];
// #[rustfmt::skip]
// pub const LIGHT_SKIN:    [Float; 3] = [0.5033493553063249, 0.5826962691691463, 0.7661554894331274];
// #[rustfmt::skip]
// pub const BLUE_SKY:      [Float; 3] = [0.6147707331960022, 0.4811322194247349, 0.3654077973601892];
// #[rustfmt::skip]
// pub const FOLIAGE:       [Float; 3] = [0.2535591668574044, 0.42406347753109025, 0.35582513160906387];
// #[rustfmt::skip]
// pub const BLUE_FLOWER:   [Float; 3] = [0.6876020447089342, 0.5040207522697795, 0.5108720531013962];
// #[rustfmt::skip]
// pub const BLUISH_GREEN:  [Float; 3] = [0.6686961165789272, 0.7477683680476082, 0.3863126573586633];
// #[rustfmt::skip]
// pub const ORANGE:        [Float; 3] = [0.17929350728618296, 0.48345159075303273, 0.8632944228274968];
// #[rustfmt::skip]
// pub const PURPLISH_BLUE: [Float; 3] = [0.6594491493095292, 0.3593957427328908, 0.2827496757457847];
// #[rustfmt::skip]
// pub const MODERATE_RED:  [Float; 3] = [0.38168917372396427, 0.3288471808957046, 0.7622034027618829];
// #[rustfmt::skip]
// pub const PURPLE:        [Float; 3] = [0.40955214770733195, 0.23208972304875258, 0.35626764324406807];
// #[rustfmt::skip]
// pub const YELLOW_GREEN:  [Float; 3] = [0.24341191729610132, 0.7404592965590906, 0.6300297550926985];
// #[rustfmt::skip]
// pub const ORANGE_YELLOW: [Float; 3] = [0.158632791638056, 0.6294804303044175, 0.8964675364309148];
// #[rustfmt::skip]
// pub const BLUE:          [Float; 3] = [0.5763637750820172, 0.24539559014267184, 0.16568245975432974];
// #[rustfmt::skip]
// pub const GREEN:         [Float; 3] = [0.2814679179064622, 0.5853513389791715, 0.283436331731136];
// #[rustfmt::skip]
// pub const RED:           [Float; 3] = [0.22203402761882962, 0.1953307392996109, 0.6867017624170291];
// #[rustfmt::skip]
// pub const YELLOW:        [Float; 3] = [0.08442816815442131, 0.7828641184100099, 0.9343099107347219];
// #[rustfmt::skip]
// pub const MAGENTA:       [Float; 3] = [0.5886472877088579, 0.3295338368810559, 0.7373464560921645];
// #[rustfmt::skip]
// pub const CYAN:          [Float; 3] = [0.6523689631494621, 0.5360036621652552, 0.0];
// #[rustfmt::skip]
// pub const WHITE:         [Float; 3] = [0.9407949950408179, 0.9618982223239491, 0.96235599298085];
// #[rustfmt::skip]
// pub const GREY_1:        [Float; 3] = [0.788189517051957, 0.7924773022049286, 0.7869840543221179];
// #[rustfmt::skip]
// pub const GREY_2:        [Float; 3] = [0.6335088120851453, 0.6350652323186083, 0.6304722667277027];
// #[rustfmt::skip]
// pub const GREY_3:        [Float; 3] = [0.47512016479743646, 0.4758831158922713, 0.4702983138780804];
// #[rustfmt::skip]
// pub const GREY_4:        [Float; 3] = [0.33357747768368046, 0.3317311360341802, 0.32654306858930343];
// #[rustfmt::skip]
// pub const BLACK:         [Float; 3] = [0.19855039291981383, 0.19627679865720607, 0.1957579919127184];