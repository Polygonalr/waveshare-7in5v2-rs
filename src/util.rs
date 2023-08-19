use image::{self, imageops::*, GenericImage, Luma, ImageBuffer};
use ril::{ BitPixel, Draw, Font, Image, TextSegment };

pub enum ColorMode {
    BlackWhite,
    BlackWhiteRed,
}

/**
 * TODO
 * 
 * Add more options such as:
 * - Cropping (whether to crop or center-and-pad)
 * - Rotation (whether to disable the automatic rotation, or allow user to specify rotation)
 * 
 * Add support for ColorMode::BlackWhiteRed
 * Reimplement with ril to support interoperability with text_to_epd
 */
pub fn image_to_epd(filepath: &str, _color_mode: ColorMode, epd_width: usize, epd_height: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if epd_width < epd_height {
        panic!("epd_width must be less than or equal to epd_height");
    }
    let mut img = image::open(filepath)?;
    if img.width() < img.height() {
        img = img.rotate90();
    }
    let img = img.resize(epd_width.try_into().unwrap(), epd_height.try_into().unwrap(), FilterType::Lanczos3);
    let img = img.grayscale();
    let mut img = img.into_luma8();
    dither(&mut img, &BiLevel);

    let mut new_canvas: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(epd_width as u32, epd_height as u32);

    // initialize canvas to white
    for pixel in new_canvas.pixels_mut() {
        *pixel = Luma([255]); 
    }

    // center-and-pad
    if img.height() < epd_height as u32 {
        new_canvas.copy_from(&img, 0, (epd_height as u32 - img.height()) / 2)?;
    } else {
        new_canvas.copy_from(&img, (epd_width as u32 - img.width()) / 2, 0)?;
    }

    let final_img = new_canvas.into_raw();
    let mut data = vec![0; final_img.len() / 8];
    for (i, byte) in data.iter_mut().enumerate() {
        for bit in 0..8 {
            if final_img[i * 8 + bit] == 0 {
                *byte |= 1 << (7 - bit);
            }
        }
    }

    assert!(data.len() == epd_height * epd_width / 8);
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
pub fn text_to_epd(text: &str, font_size: f32, width: usize, height: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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