pub const SPACER: u8 = 0;
pub const END_CHAR: char = 0 as char;
pub const MAX_CHAR: usize = u16::MAX as usize;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SIGNATURE: &[u8; 7] = &[b'j', b'w', b's', b'h', b'u', b'f', b'f'];