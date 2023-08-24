use crate::epd_configs::{Action, EpdConfig};

pub const EPD_CONFIG: EpdConfig = EpdConfig {
    init_commands: &[
        Action::SendCommand(0x06),
        Action::SendData(&[0x17, 0x17, 0x28, 0x17]),
        Action::SendCommand(0x01),
        Action::SendData(&[0x07, 0x07, 0x3f, 0x3f]),
        Action::SendCommand(0x04),
        Action::ReadBusy,
        Action::SendCommand(0x00),
        Action::SendData(&[0x1f]),
        Action::SendCommand(0x61),
        Action::SendData(&[0x03, 0x20, 0x01, 0xE0]),
        Action::SendCommand(0x15),
        Action::SendData(&[0x00]),
        Action::SendCommand(0x50),
        Action::SendData(&[0x10, 0x07]),
        Action::SendCommand(0x60),
        Action::SendData(&[0x22]),
    ],
    width: 800,
    height: 480,
};
