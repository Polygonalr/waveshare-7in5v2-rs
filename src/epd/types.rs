#[derive(Debug, Default, Clone)]
pub struct EpdConfig {
    pub(crate) init_commands: &'static [Action],
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    SendCommand(u8),
    SendData(&'static [u8]),
    ReadBusy,
    Delay(u64),
}
