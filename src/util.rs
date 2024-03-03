use core::ops::Shl;

pub trait ConvertToU16 {
    fn to_u16_big_endian(self) -> [u16; 4];
    fn to_u16_little_endian(self) -> [u16; 4];
}

impl ConvertToU16 for [u8; 8] {
    fn to_u16_big_endian(self) -> [u16; 4] {
        [
            ((self[0] as u16) << 8) | self[1] as u16,
            ((self[2] as u16) << 8) | self[3] as u16,
            ((self[4] as u16) << 8) | self[5] as u16,
            ((self[6] as u16) << 8) | self[7] as u16,
        ]
    }

    fn to_u16_little_endian(self) -> [u16; 4] {
        [
            ((self[1] as u16) << 8) | self[0] as u16,
            ((self[3] as u16) << 8) | self[2] as u16,
            ((self[5] as u16) << 8) | self[4] as u16,
            ((self[7] as u16) << 8) | self[6] as u16,
        ]
    }
}

pub trait ConvertToU8 {
    fn to_u8_big_endian(self) -> [u8; 8];
    fn to_u8_little_endian(self) -> [u8; 8];
}

impl ConvertToU8 for [u16; 4] {
    fn to_u8_big_endian(self) -> [u8; 8] {
        [
            (self[0] >> 8) as u8,
            self[0] as u8,
            (self[1] >> 8) as u8,
            self[1] as u8,
            (self[2] >> 8) as u8,
            self[2] as u8,
            (self[3] >> 8) as u8,
            self[3] as u8,
        ]
    }

    fn to_u8_little_endian(self) -> [u8; 8] {
        [
            self[0] as u8,
            (self[0] >> 8) as u8,
            self[1] as u8,
            (self[1] >> 8) as u8,
            self[2] as u8,
            (self[2] >> 8) as u8,
            self[3] as u8,
            (self[3] >> 8) as u8,
        ]
    }
}

pub fn make_u16(upper: u8, lower: u8) -> u16 {
    lower as u16 | (upper as u16).shl(8)
}

pub fn make_u32(top: u8, high: u8, low: u8, bottom: u8) -> u32 {
    bottom as u32 | (low as u32).shl(8) | (high as u32).shl(16) | (top as u32).shl(24)
}

pub fn make_i16(upper: u8, lower: u8) -> i16 {
    make_u16(upper, lower) as i16
}
