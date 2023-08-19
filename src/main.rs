use clap::Parser;
use waveshare_rpi::{epd::epd7in5_v2::EPD_CONFIG, Epd};
use waveshare_rpi::util::{ColorMode, image_to_epd, text_to_epd};

/// Program to update a Waveshare 7.5" e-ink display
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the image to display
    #[arg(short, long)]
    image: Option<String>,

    /// Text to display
    #[arg(short, long)]
    text: Option<String>,

    /// Clear the display
    #[arg(short, long)]
    clear: bool,
}

fn main() {
    let args = Args::parse();

    match args.image {
        Some(filepath) => {
            let data = image_to_epd(&filepath, ColorMode::BlackWhite, EPD_CONFIG.width, EPD_CONFIG.height).unwrap();
            let mut epd = Epd::new(EPD_CONFIG);
            epd.display(&data);
            return;
        },
        None => ()
    }

    match args.text {
        Some(text) => {
            let data = text_to_epd(&text, 24.0, EPD_CONFIG.width, EPD_CONFIG.height).unwrap();
            let mut epd = Epd::new(EPD_CONFIG);
            epd.display(&data);
            return;
        },
        None => ()
    }

    match args.clear {
        true => {
            let mut epd = Epd::new(EPD_CONFIG);
            epd.clear();
            return;
        },
        false => ()
    }

    println!("No image or text specified. Use --help for usage information.");
}
