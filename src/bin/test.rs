use cgmath::{Angle, Rad, Rotation3};
use rust_v2::Rot3;
use serde::{Deserialize, Serialize};

fn main() {
    let rot = Rot3::from_angle_x(Rad::turn_div_4());

    println!("{}", ron::to_string(&rot).unwrap());
}
