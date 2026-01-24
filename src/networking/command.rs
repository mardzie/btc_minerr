pub type CommandBytes = [u8; 12];

pub const VERSION_BYTES: CommandBytes = [
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const VERACK_BYTES: CommandBytes = [
    0x76, 0x65, 0x72, 0x61, 0x63, 0x6B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Version,
    Verack,
}

impl Command {
    pub fn from_bytes(bytes: &CommandBytes) -> Self {
        match bytes {
            &VERSION_BYTES => Self::Version,
            &VERACK_BYTES => Self::Verack,
            _ => panic!("Unknown command!"),
        }
    }

    pub const fn to_bytes(&self) -> [u8; 12] {
        match self {
            Self::Version => VERSION_BYTES,
            Self::Verack => VERACK_BYTES,
        }
    }
}
