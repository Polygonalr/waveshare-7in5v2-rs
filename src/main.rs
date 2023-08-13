use waveshare_7in5v2_rs::{epd::epd7in5_v2::EPD_CONFIG, Epd};

fn main() {
    let mut e = Epd::new(EPD_CONFIG);
    e.clear();
}
