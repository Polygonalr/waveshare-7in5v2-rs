/*!
This crate provides a high-level interface to the Waveshare e-paper displays for the Raspberry Pi. It is based on
Waveshare's C library for the Raspberry Pi[^1] with some modifications to make it more idiomatic.
`waveshare-rpi` also provides a [`converter`] module to convert images and text to the format for use
with the e-paper displays.

# Supported Waveshare E-Paper Display Models

| Model | Tested |
|---|---|
| [Waveshare 7.5inch V2 display](https://www.waveshare.com/7.5inch-e-Paper-HAT.htm) | Yes |

# Example

Usage with the 7.5" V2 display:

```no_run
use waveshare_rpi::{epd_configs::epd7in5_v2::EPD_CONFIG, Epd};
use waveshare_rpi::converter::{ColorMode, EpdImageOptions, image_to_epd};

// Initialize the interface to interact with the epd7in5_v2 display
let mut my_epd = Epd::new(EPD_CONFIG);

// Initialize the image options for the image to be displayed on the display
let mut image_options = EpdImageOptions::new();
image_options.load_epd_config(EPD_CONFIG);

// Resizes and converts image.png into the format compatible with the epd7in5_v2 display
let img_data = image_to_epd("image.png", image_options).unwrap();

// Transfer the image data to the display for displaying
my_epd.display(&img_data).unwrap();
```

[^1]: [https://github.com/waveshareteam/e-Paper](https://github.com/waveshareteam/e-Paper)
 */

pub mod converter;
pub mod epd_configs;
mod rpi_helper;

use epd_configs::{Action, EpdConfig};
use rpi_helper::RpiGpio;
use rppal::gpio::Level;
use std::thread::sleep;
use std::time::Duration;

const DATA_BUFFER_SIZE: usize = 4096;

/// Error returned when the size of the image data does not match the EPD's config.
#[derive(Debug, Clone)]
pub struct ImgSizeMismatchError;

/// Represents a E-Paper Display.
pub struct Epd {
    config: EpdConfig,
    rpi: RpiGpio,
}

impl Epd {
    /// Creates a new instance of `Epd` with the config of a Waveshare E-Paper Display.
    pub fn new(config: EpdConfig) -> Self {
        let rpi = RpiGpio::new();
        let mut s = Self { config, rpi };
        s.init();
        s
    }

    /// Sends commands to the EPD to initialize it.
    pub fn init(&mut self) {
        simple_logger::SimpleLogger::new().env().init().unwrap();
        log::info!("Initializing display!");
        self.reset();
        for &command in self.config.init_commands {
            match command {
                Action::SendCommand(command) => {
                    self.send_command(command);
                }
                Action::SendData(data) => {
                    self.send_data(data);
                }
                Action::ReadBusy => {
                    self.read_busy();
                }
                Action::Delay(ms) => {
                    sleep(Duration::from_millis(ms));
                }
            }
        }
    }

    /// Returns the number of bytes that the EPD takes in for displaying an image.
    pub fn image_buffer_size(&self) -> usize {
        self.config.height * self.config.width / 8
    }

    fn reset(&mut self) {
        self.rpi.gpio.rst.set_high();
        sleep(Duration::from_millis(20));
        self.rpi.gpio.rst.set_low();
        sleep(Duration::from_millis(2));
        self.rpi.gpio.rst.set_high();
        sleep(Duration::from_millis(20));
    }

    fn send_command(&mut self, command: u8) {
        self.rpi.gpio.dc.set_low();
        self.rpi.gpio.cs.set_low();
        self.rpi.spi.write(&[command]).unwrap();
        self.rpi.gpio.cs.set_high();
    }

    fn send_data(&mut self, data: &[u8]) {
        let chunks = data.chunks(DATA_BUFFER_SIZE);
        for chunk in chunks {
            self.rpi.gpio.dc.set_high();
            self.rpi.gpio.cs.set_low();
            self.rpi.spi.write(chunk).unwrap();
            self.rpi.gpio.cs.set_high();
        }
    }

    /// Constantly read from the busy pin and returns once the EPD stops being busy.
    pub fn read_busy(&mut self) {
        log::info!("Waiting until EPD is no longer busy");
        self.send_command(0x71);
        let mut busy = self.rpi.gpio.busy.read();
        while busy == Level::Low {
            sleep(Duration::from_millis(100));
            busy = self.rpi.gpio.busy.read();
        }
        log::info!("EPD is no longer busy");
    }

    /// Clears the screen by setting it all pixels to wwhite
    pub fn clear(&mut self) {
        // TODO support Black&White&Red displays
        log::info!("Clearing EPD");
        self.send_command(0x10);
        let blank = vec![0x00; self.image_buffer_size()];
        self.send_data(&blank);
        self.send_command(0x13);
        self.send_data(&blank);
        self.send_command(0x12);
        sleep(Duration::from_millis(100));
        self.read_busy();
    }

    /// Takes in image data (represented in an array of `u8`) and displays it on the EPD.
    /// Returns `Err(ImgSizeMismatchError)` if the size of image data does not match the EPD's config.
    ///
    /// # Image data format for Black & White displays
    ///
    /// Each bit in the `u8` array represents a pixel. If the bit is set, the pixel will be black.
    /// Likewise if the bit is unset, the pixel will be white. The EPD will draw from left to right
    /// based on the input array starting from the top-left, and will wrap back to the left side of
    /// the next row when it reaches the right side of the current row
    pub fn display(&mut self, data: &[u8]) -> Result<(), ImgSizeMismatchError> {
        if data.len() != self.image_buffer_size() {
            return Err(ImgSizeMismatchError);
        }
        log::info!("Displaying image on EPD");
        self.send_command(0x13);
        self.send_data(data);
        self.send_command(0x12);
        sleep(Duration::from_millis(100));
        self.read_busy();
        Ok(())
    }

    /// Puts the display to a low power consumption state.
    pub fn sleep(&mut self) {
        log::info!("Sleeping EPD");
        self.send_command(0x02);
        self.read_busy();
        self.send_command(0x07);
        self.send_data(&[0xA5]);
        sleep(Duration::from_millis(1500));
    }
}

impl Drop for Epd {
    fn drop(&mut self) {
        self.sleep();
    }
}

impl Default for Epd {
    fn default() -> Self {
        use epd_configs::epd7in5_v2::EPD_CONFIG;
        Self::new(EPD_CONFIG)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_test() {
        use epd_configs::epd7in5_v2::EPD_CONFIG;
        let mut epd = Epd::new(EPD_CONFIG);
        epd.clear();
    }
}
