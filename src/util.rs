use image::{self, imageops::*, GenericImage, Luma, ImageBuffer};

pub enum ColorMode {
    BlackWhite,
    BlackWhiteRed,
}

// Only supports BlackWhite for now, will add BlackWhiteRed later?
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
    // TODO add a toggle for default canvas color, and whether to crop-to-fit or center-and-pad
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