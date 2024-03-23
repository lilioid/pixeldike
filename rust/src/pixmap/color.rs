use std::fmt::{Formatter, UpperHex};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

/// Color data represented as red, green, and blue channels each having a depth of 8 bits
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
#[repr(C)]
pub struct Color(u32);

impl From<[u8; 3]> for Color {
    fn from(data: [u8; 3]) -> Self {
        Self(u32::from_be_bytes([data[0], data[1], data[2], 0]))
    }
}

impl From<Color> for [u8; 3] {
    fn from(value: Color) -> Self {
        let channels = value.0.to_be_bytes();
        [channels[0], channels[1], channels[2]]
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(data: (u8, u8, u8)) -> Self {
        Self(u32::from_be_bytes([data.0, data.1, data.2, 0]))
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        let channnels = value.0.to_be_bytes();
        (channnels[0], channnels[1], channnels[2])
    }
}

impl From<u32> for Color {
    fn from(src: u32) -> Self {
        Self(src)
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<Color> for Vec<u8> {
    fn from(value: Color) -> Self {
        let channels = value.0.to_be_bytes();
        vec![channels[0], channels[1], channels[2]]
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        let channels: [u8; 3] = (*self).into();
        format!("#{:02X}{:02X}{:02X}", channels[0], channels[1], channels[2])
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let channels: [u8; 3] = (*self).into();
        f.write_fmt(format_args!(
            "{:02X}{:02X}{:02X}",
            channels[0], channels[1], channels[2]
        ))
    }
}

#[cfg(test)]
impl Arbitrary for Color {
    fn arbitrary(g: &mut Gen) -> Self {
        u32::arbitrary(g).into()
    }
}

#[cfg(test)]
quickcheck! {
    fn test_u32_conversion(color: Color) -> bool {
        let c_enc: u32 = color.into();
        let c_dec: Color = Color::from(c_enc);
        c_dec == color
    }
}
