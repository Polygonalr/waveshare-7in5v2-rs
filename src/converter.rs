//! Contains functions for converting images and text to EPD format.
use crate::EpdConfig;
use image::{self, imageops::*, DynamicImage, GenericImage, ImageBuffer, Luma};
use ril::{BitPixel, Draw, Font, Image, TextSegment};

/// Color mode for the converted image data. Currently unutilized.
#[derive(Default, PartialEq)]
pub enum ColorMode {
    /// For displays which only displays black and white.
    #[default]
    BlackWhite,
    /// For displays which displays black, white and red.
    BlackWhiteRed,
}

/// Cropping mode for converting images to EPD format.
#[derive(Default, PartialEq)]
pub enum CropMode {
    /// Resize to fit the image in the center of the display and pad the rest of the space with white.
    #[default]
    Center,
    /// Resize the image and crop it to fit the display with no padding.
    CropToFit,
}

/// Rotation mode for converting images to EPD format.
#[derive(Default, PartialEq)]
pub enum RotationMode {
    /// Automatically rotate the image if the width is less than the height.
    #[default]
    Automatic,
    /// Force the image to be displayed in landscape mode.
    ForceLandscape,
    /// Force the image to be displayed in portrait mode.
    ForcePortrait,
}

/// Options for image_to_epd.
///
/// To initialize it, you can either use the new() function and set the options manually,
/// or declare the struct while setting the options you want to change and using the
/// default values for the rest via the Default trait.
///
/// # Available Options
///
/// | Option name | Type | Description | Default value |
/// |---|---|---|---|
/// | `crop_mode` | [`CropMode`] | Modes to pre-process the image to fit on the display. | [`Center`](CropMode::Center) |
/// | `rotation_mode` | [`RotationMode`] | How to rotate the image before pre-processing. | [`Automatic`](RotationMode::Automatic) |
/// | `color_mode` | [`ColorMode`] | Unutilized at the moment, sets the color mode of the display. | [`BlackWhite`](ColorMode::BlackWhite) |
/// | `epd_width` | `usize` | Width of the EPD display measured in pixels. **Do not set this value explicitly!**. Use `load_epd_config` instead if you want to set this value. | 0 |
/// | `epd_height` | `usize` | Height of the EPD display measured in pixels. **Do not set this value explicitly!**. Use `load_epd_config` instead if you want to set this value. | 0 |
///
/// # Examples
///
/// **Using `EpdImageOptions::new()`:**
///
/// ```
/// use waveshare_rpi::epd_configs::epd7in5_v2::EPD_CONFIG;
/// use waveshare_rpi::converter::{EpdImageOptions, CropMode, RotationMode};
///
/// let mut options = EpdImageOptions::new();
/// options.crop_mode = CropMode::CropToFit;
/// options.rotation_mode = RotationMode::ForceLandscape;
/// options.load_epd_config(EPD_CONFIG);
/// ```
///
/// **Using `Default::default()`:**
///
/// ```
/// use waveshare_rpi::epd_configs::epd7in5_v2::EPD_CONFIG;
/// use waveshare_rpi::converter::{EpdImageOptions, CropMode, RotationMode};
///
/// let mut options = EpdImageOptions {
///   crop_mode: CropMode::CropToFit,
///   rotation_mode: RotationMode::ForceLandscape,
///   ..Default::default()
/// };
/// options.load_epd_config(EPD_CONFIG);
/// ```
#[derive(Default, PartialEq)]
pub struct EpdImageOptions {
    pub crop_mode: CropMode,
    pub rotation_mode: RotationMode,
    pub color_mode: ColorMode,
    pub epd_width: usize,
    pub epd_height: usize,
}

impl EpdImageOptions {
    /// Creates a new EpdImageOptions struct with default values.
    pub fn new() -> EpdImageOptions {
        Default::default()
    }

    /// Update a new EpdImageOptions struct with the width and height of the display from its config.
    pub fn load_epd_config(&mut self, epd_config: EpdConfig) {
        self.epd_width = epd_config.width;
        self.epd_height = epd_config.height;
    }

    /// Returns true if the image needs to be rotated.
    fn check_rotation(&self, img: &DynamicImage) -> bool {
        match self.rotation_mode {
            RotationMode::Automatic => {
                (img.width() > img.height() && self.epd_width < self.epd_height)
                    || (img.width() < img.height() && self.epd_width > self.epd_height)
            }
            RotationMode::ForcePortrait => {
                img.width() > img.height() || self.epd_width < self.epd_height
            }
            RotationMode::ForceLandscape => {
                img.width() < img.height() || self.epd_width > self.epd_height
            }
        }
    }
}

fn center_and_pad(options: &EpdImageOptions, img: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    // Process the image
    let img = img.resize(
        options.epd_width.try_into().unwrap(),
        options.epd_height.try_into().unwrap(),
        FilterType::Lanczos3,
    );
    let img = img.grayscale();
    let mut img = img.into_luma8();
    dither(&mut img, &BiLevel);

    let mut new_canvas: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(options.epd_width as u32, options.epd_height as u32);

    // initialize canvas to white
    for pixel in new_canvas.pixels_mut() {
        *pixel = Luma([255]);
    }

    if img.height() < options.epd_height as u32 {
        new_canvas
            .copy_from(&img, 0, (options.epd_height as u32 - img.height()) / 2)
            .unwrap();
    } else {
        new_canvas
            .copy_from(&img, (options.epd_width as u32 - img.width()) / 2, 0)
            .unwrap();
    }

    new_canvas
}

fn crop_to_fit(options: &EpdImageOptions, img: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let img = img.resize_to_fill(
        options.epd_width.try_into().unwrap(),
        options.epd_height.try_into().unwrap(),
        FilterType::Lanczos3,
    );
    let img = img.grayscale();
    let mut img = img.into_luma8();
    dither(&mut img, &BiLevel);
    img
}

/// Convert an image to EPD format to be displayed on the e-paper display. Uses the
/// [`image`] crate to resize, dither and optionally crop images.
///
/// # Arguments
///
/// * `filepath` - The path to the image file.
/// * `options` - The options for converting the image of type [`EpdImageOptions`]. See the struct's documentation for more details.
///
/// # Examples
///
/// ```no_run
/// use waveshare_rpi::epd_configs::epd7in5_v2::EPD_CONFIG;
/// use waveshare_rpi::converter::{EpdImageOptions, CropMode, RotationMode};
///
/// let mut options = EpdImageOptions::new();
/// options.load_epd_config(EPD_CONFIG);
/// let data = waveshare_rpi::converter::image_to_epd("test.jpg", options).unwrap();
/// ```
///
/// # To-dos
///
/// * Add support for `ColorMode::BlackWhiteRed`.
/// * Reimplement with ril to support interoperability with `text_to_epd`.
/// * Integrate this function into te Epd struct via a trait (toggleable with a feature).
pub fn image_to_epd(
    filepath: &str,
    options: EpdImageOptions,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if options.epd_width == 0 || options.epd_height == 0 {
        return Err("epd_width and epd_height must be greater than 0".into());
    }

    let mut img = image::open(filepath)?;

    // rotate image if necessary
    if options.check_rotation(&img) {
        img = img.rotate90();
    }

    let img = match options.crop_mode {
        CropMode::Center => center_and_pad(&options, img),
        CropMode::CropToFit => crop_to_fit(&options, img),
    };

    // convert to epd format
    let final_img = img.into_raw();
    let mut data = vec![0; final_img.len() / 8];
    for (i, byte) in data.iter_mut().enumerate() {
        for bit in 0..8 {
            if final_img[i * 8 + bit] == 0 {
                *byte |= 1 << (7 - bit);
            }
        }
    }

    assert!(data.len() == options.epd_height * options.epd_width / 8);
    Ok(data)
}

/// Convert text to EPD format to be displayed on the e-paper display. Uses the [`ril`]
/// as the backend to render text.
///
/// # Arguments
///
/// # Examples
///
/// # To-dos
///
/// - Add more options such as:
///   - Font file
///   - Font size
///   - Alignment/Centering
///   - Support for ColorMode
/// - Ensure the text will fit on the display (and add support for text wrapping)
pub fn text_to_epd(
    text: &str,
    font_size: f32,
    width: usize,
    height: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let default_font_file = include_bytes!("fonts/Roboto-Regular.ttf") as &[u8];
    let font = Font::from_bytes(default_font_file, font_size).unwrap();
    let mut image = Image::new(width as u32, height as u32, BitPixel::new(true));
    TextSegment::new(&font, text, BitPixel::new(false))
        .with_position(0, 0)
        .draw(&mut image);

    let mut data = vec![0; image.data.len() / 8];
    for (i, byte) in data.iter_mut().enumerate() {
        for bit in 0..8 {
            if !image.data[i * 8 + bit].value() {
                *byte |= 1 << (7 - bit);
            }
        }
    }
    Ok(data)
}
