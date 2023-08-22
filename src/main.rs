use clap::Parser;
use waveshare_rpi::util::{image_to_epd, text_to_epd, EpdImageOptions};
use waveshare_rpi::{epd::epd7in5_v2::EPD_CONFIG, Epd};

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

    if let Some(filepath) = args.image {
        let image_options = EpdImageOptions {
            epd_width: EPD_CONFIG.width,
            epd_height: EPD_CONFIG.height,
            ..Default::default()
        };
        let data = image_to_epd(&filepath, image_options).unwrap();
        let mut epd = Epd::new(EPD_CONFIG);
        epd.display(&data);
        return;
    }

    if let Some(text) = args.text {
        let data = text_to_epd(&text, 24.0, EPD_CONFIG.width, EPD_CONFIG.height).unwrap();
        let mut epd = Epd::new(EPD_CONFIG);
        epd.display(&data);
        return;
    }

    if args.clear {
        let mut epd = Epd::new(EPD_CONFIG);
        epd.clear();
        return;
    }

    println!("No image or text specified. Use --help for usage information.");
}
