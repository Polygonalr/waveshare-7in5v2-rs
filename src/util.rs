use crate::EpdConfig;
use image::{self, imageops::*, DynamicImage, GenericImage, ImageBuffer, Luma};
use ril::{BitPixel, Draw, Font, Image, TextSegment};

#[derive(Default, PartialEq)]
pub enum ColorMode {
    #[default]
    BlackWhite,
    BlackWhiteRed,
}

#[derive(Default, PartialEq)]
pub enum CropMode {
    #[default]
    Center,
    CropToFit,
}

#[derive(Default, PartialEq)]
pub enum RotationMode {
    #[default]
    Automatic,
    ForceHorizontal,
    ForceVertical,
}

#[derive(Default, PartialEq)]
pub struct EpdImageOptions {
    pub crop_mode: CropMode,
    pub rotation_mode: RotationMode,
    pub color_mode: ColorMode,
    pub epd_width: usize,
    pub epd_height: usize,
}

impl EpdImageOptions {
    pub fn new(
        epd_config: EpdConfig,
        crop_mode: CropMode,
        rotation_mode: RotationMode,
        color_mode: ColorMode,
    ) -> Self {
        Self {
            crop_mode,
            rotation_mode,
            color_mode,
            epd_width: epd_config.width,
            epd_height: epd_config.height,
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

/**
 * TODO
 *
 * Add support for ColorMode::BlackWhiteRed
 * Reimplement with ril to support interoperability with text_to_epd
 */
pub fn image_to_epd(
    filepath: &str,
    options: EpdImageOptions,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if options.epd_width < options.epd_height {
        return Err("epd_width must be less than or equal to epd_height".into());
    } else if options.epd_width == 0 || options.epd_height == 0 {
        return Err("epd_width and epd_height must be greater than 0".into());
    }

    let mut img = image::open(filepath)?;
    if (options.rotation_mode == RotationMode::Automatic && img.width() < img.height())
        || options.rotation_mode == RotationMode::ForceVertical
    {
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

/**
 * TODO
 *
 * Add more options such as:
 * - Font file
 * - Alignment/Centering
 * - Support for ColorMode
 *
 * Ensure the text will fit on the display (and add support for text wrapping)
 */
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

    image.save(ril::ImageFormat::Jpeg, "test.jpg").unwrap();
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
