use crate::epd_configs::{Action, EpdConfig};

pub const EPD_CONFIG: EpdConfig = EpdConfig {
    init_commands: &[
        Action::SendCommand(0x04),
        Action::ReadBusy,
        Action::SendCommand(0x00),
        Action::SendData(&[0x1f]),
        Action::SendCommand(0x61),
        Action::SendData(&[0x08, 0x01, 0x28]),
        Action::SendCommand(0x50),
        Action::SendData(&[0x97]),
    ],
    width: 128,
    height: 296,
};
