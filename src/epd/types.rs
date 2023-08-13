#[derive(Debug, Clone)]
pub struct EpdConfig {
    pub(crate) init_commands: &'static [Action],
    pub(crate) width: usize,
    pub(crate) height: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    SendCommand(u8),
    SendData(&'static [u8]),
    ReadBusy,
    Delay(u64),
}
