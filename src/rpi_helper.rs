use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

// RPi constants
pub const RST_PIN: u8 = 17;
pub const DC_PIN: u8 = 25;
pub const CS_PIN: u8 = 8;
pub const BUSY_PIN: u8 = 24;
pub const PWR_PIN: u8 = 18;

pub struct RpiGpioPins {
    pub rst: OutputPin,
    pub dc: OutputPin,
    pub cs: OutputPin,
    pub busy: InputPin,
    pub pwr: OutputPin,
}

pub struct RpiGpio {
    pub gpio: RpiGpioPins,
    pub spi: Spi,
}

impl RpiGpio {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 4_000_000, Mode::Mode0).unwrap();
        let mut pwr = gpio.get(PWR_PIN).unwrap().into_output();
        pwr.set_high();
        let rst = gpio.get(RST_PIN).unwrap().into_output();
        let dc = gpio.get(DC_PIN).unwrap().into_output();
        let cs = gpio.get(CS_PIN).unwrap().into_output();
        let busy = gpio.get(BUSY_PIN).unwrap().into_input();
        RpiGpio {
            gpio: RpiGpioPins {
                rst,
                dc,
                cs,
                busy,
                pwr,
            },
            spi,
        }
    }
}

impl Drop for RpiGpio {
    fn drop(&mut self) {
        self.gpio.rst.set_low();
        self.gpio.dc.set_low();
        self.gpio.pwr.set_low();
    }
}
