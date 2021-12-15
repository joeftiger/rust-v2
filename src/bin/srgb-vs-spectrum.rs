use image::Rgb;
use show_image::{create_window, ImageInfo, ImageView, WindowOptions};
use show_image::event::{VirtualKeyCode, WindowEvent};
use rust_v2::color::{Color, Srgb};
use rust_v2::Spectrum;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = WindowOptions::default().set_size([500, 500]);
    let window = create_window("Test", options)?;
    let colors = Color::variants();
    let mut i = 0;
    let mut mul = 1.0;
    println!("Left:  Srgb\nRight: Srgb from Spectrum");
    loop {
        let c = colors[i];
        let srgb: Rgb<u8> = (Srgb::from(c) * mul).into();
        let spectrum: Rgb<u8> = (Spectrum::from(c) * mul).into();

        let data = [
            srgb[0],
            srgb[1],
            srgb[2],
            spectrum[0],
            spectrum[1],
            spectrum[2],
        ];

        let image = ImageView::new(ImageInfo::rgb8(2, 1), &data);
        window.set_image(format!("{:?}", c), image)?;
        for event in window.event_channel()? {
            if let WindowEvent::KeyboardInput(event) = event {
                if event.input.state.is_pressed() {
                    match event.input.key_code {
                        Some(VirtualKeyCode::Escape) => return Ok(()),
                        Some(VirtualKeyCode::Left) => {
                            i = if i == 0 { i } else { i - 1 };
                            break;
                        }
                        Some(VirtualKeyCode::Right) => {
                            i = if i == (colors.len() - 1) { i } else { i + 1 };
                            break;
                        }
                        Some(VirtualKeyCode::Up) => {
                            mul *= 1.25;
                            break;
                        }
                        Some(VirtualKeyCode::Down) => {
                            mul /= 1.25;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}