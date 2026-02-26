/// Firmware version struct shared across all PAC firmwares.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirmwareVersion {
    pub major: u16,
    pub minor: u16,
}

impl FirmwareVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }
}
