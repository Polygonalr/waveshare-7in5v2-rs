use std::env::args;
use waveshare_rpi::{epd::epd7in5_v2::EPD_CONFIG, Epd};
use waveshare_rpi::util::{ColorMode, image_to_epd};

fn main() {
    let mut e = Epd::new(EPD_CONFIG);
    if args().len() > 1 {
        let data = image_to_epd(&args().nth(1).unwrap(), ColorMode::BlackWhite, EPD_CONFIG.width, EPD_CONFIG.height).unwrap();
        e.display(&data);
        return;
    } else {
        e.clear();
    }
}
