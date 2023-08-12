mod rpi_helper;
use rpi_helper::RpiGpio;
use rppal::gpio::Level;
use std::thread::sleep;
use std::time::Duration;
use log::info;

const EPD_WIDTH: usize = 800;
const EPD_HEIGHT: usize = 480; 
const DATA_BUFFER_SIZE: usize = 4096;

pub struct Epd {
    rpi: RpiGpio,
}

impl Epd {
    pub fn new() -> Self {
        let rpi = RpiGpio::new();
        let mut s = Self { rpi };
        s.init();
        s
    }

    pub fn init(&mut self) {
        info!("Initializing display!");
        self.reset();

        // btst
        self.send_command(0x06);
        self.send_data(&[0x17, 0x17, 0x28, 0x17]);
        
        // power setting
        self.send_command(0x01);
        self.send_data(&[0x07, 0x07, 0x3f, 0x3f]);

        // power on
        self.send_command(0x04);
        sleep(Duration::from_millis(100));

        self.read_busy();

        // panel setting
        self.send_command(0x00);
        self.send_data(&[0x1F]);

        // tres
        self.send_command(0x61);
        self.send_data(&[0x03, 0x20, 0x01, 0xE0]);

        // soft start
        self.send_command(0x15);
        self.send_data(&[0x00]);

        // vcom and data interval setting
        self.send_command(0x50);
        self.send_data(&[0x10, 0x07]);

        // TCON setting
        self.send_command(0x60);
        self.send_data(&[0x22]);
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
        info!("Waiting until EPD is no longer busy");
        self.send_command(0x71);
        let mut busy = self.rpi.gpio.busy.read();
        while busy == Level::Low {
            sleep(Duration::from_millis(100));
            busy = self.rpi.gpio.busy.read();
        }
        info!("EPD is no longer busy");
    }

    pub fn clear(&mut self) {
        info!("Clearing EPD");
        self.send_command(0x10);
        self.send_data(&[0x00; EPD_HEIGHT * EPD_WIDTH / 8]);
        self.send_command(0x13);
        self.send_data(&[0x00; EPD_HEIGHT * EPD_WIDTH / 8]);
        self.send_command(0x12);
        sleep(Duration::from_millis(100));
        self.read_busy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_test() {
        let mut epd = Epd::new();
        epd.clear();
    }
}
