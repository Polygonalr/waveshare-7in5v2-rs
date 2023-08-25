//! Contains the configurations for different Waveshare e-ink display models.
pub mod epd7in5_v2;

/// Represents the configuration of a Waveshare e-ink display model.
/// 
/// Todo: Add partial refresh and more available color modes.
#[derive(Debug, Default, Clone)]
pub struct EpdConfig {
    pub(crate) init_commands: &'static [Action],
    pub width: usize,
    pub height: usize,
}

/// Possible actions to execute to the Waveshare E-Paper Driver HAT.
#[derive(Debug, Clone, Copy)]
pub enum Action {
    SendCommand(u8),
    SendData(&'static [u8]),
    ReadBusy,
    Delay(u64),
}
