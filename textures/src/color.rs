use bincode::{Decode, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Decode, Encode)]
pub struct Color16(pub u16);
