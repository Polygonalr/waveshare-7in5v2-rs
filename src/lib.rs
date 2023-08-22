pub mod epd;
mod rpi_helper;
pub mod util;

use epd::types::{Action, EpdConfig};
use rpi_helper::RpiGpio;
use rppal::gpio::Level;
use std::thread::sleep;
use std::time::Duration;

const DATA_BUFFER_SIZE: usize = 4096;

pub struct Epd {
    config: EpdConfig,
    rpi: RpiGpio,
}

impl Epd {
    pub fn new(config: EpdConfig) -> Self {
        let rpi = RpiGpio::new();
        let mut s = Self { config, rpi };
        s.init();
        s
    }

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

    fn image_buffer_size(&self) -> usize {
        self.config.height * self.config.width / 8
    }

    pub fn reset(&mut self) {
        self.rpi.gpio.rst.set_high();
        sleep(Duration::from_millis(20));
        self.rpi.gpio.rst.set_low();
        sleep(Duration::from_millis(2));
        self.rpi.gpio.rst.set_high();
        sleep(Duration::from_millis(20));
    }

    pub fn send_command(&mut self, command: u8) {
        self.rpi.gpio.dc.set_low();
        self.rpi.gpio.cs.set_low();
        self.rpi.spi.write(&[command]).unwrap();
        self.rpi.gpio.cs.set_high();
    }

    pub fn send_data(&mut self, data: &[u8]) {
        let chunks = data.chunks(DATA_BUFFER_SIZE);
        for chunk in chunks {
            self.rpi.gpio.dc.set_high();
            self.rpi.gpio.cs.set_low();
            self.rpi.spi.write(chunk).unwrap();
            self.rpi.gpio.cs.set_high();
        }
    }

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

    pub fn clear(&mut self) {
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

    pub fn display(&mut self, data: &[u8]) {
        if data.len() != self.image_buffer_size() {
            panic!("Data size does not match display size");
        }
        log::info!("Displaying image on EPD");
        self.send_command(0x13);
        self.send_data(data);
        self.send_command(0x12);
        sleep(Duration::from_millis(100));
        self.read_busy();
    }

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
        use epd::epd7in5_v2::EPD_CONFIG;
        Self::new(EPD_CONFIG)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_test() {
        use epd::epd7in5_v2::EPD_CONFIG;
        let mut epd = Epd::new(EPD_CONFIG);
        epd.clear();
    }
}
