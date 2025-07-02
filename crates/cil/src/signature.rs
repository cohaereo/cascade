use std::fmt::Display;

use binrw::binread;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SignatureKind {
    StandaloneMethod,
    Field,
    LocalVar,
    Property,
}

impl TryFrom<u8> for SignatureKind {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0xF {
            0x0..=0x5 => Ok(SignatureKind::StandaloneMethod),
            0x6 => Ok(SignatureKind::Field),
            0x7 => Ok(SignatureKind::LocalVar),
            0x8 => Ok(SignatureKind::Property),
            _ => Err("Invalid signature kind"),
        }
    }
}

#[binread]
#[br(repr(u8))]
#[derive(Debug, PartialEq)]
pub enum Element {
    End = 0x00,
    Void = 0x01,
    Boolean = 0x02,
    Char = 0x03,
    I1 = 0x04,
    U1 = 0x05,
    I2 = 0x06,
    U2 = 0x07,
    I4 = 0x08,
    U4 = 0x09,
    I8 = 0x0A,
    U8 = 0x0B,
    R4 = 0x0C,
    R8 = 0x0D,
    // String = 0x0E,
    // Ptr = 0x0F,
    // ByRef = 0x10,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Element::End => "<end>",
            Element::Void => "void",
            Element::Boolean => "bool",
            Element::Char => "char",
            Element::I1 => "int8",
            Element::U1 => "uint8",
            Element::I2 => "int16",
            Element::U2 => "uint16",
            Element::I4 => "int32",
            Element::U4 => "uint32",
            Element::I8 => "int64",
            Element::U8 => "uint64",
            Element::R4 => "float32",
            Element::R8 => "float64",
        };

        f.write_str(s)
    }
}
