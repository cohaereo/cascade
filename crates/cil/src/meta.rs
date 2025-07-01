use std::fmt::{Debug, Display};

use binrw::{NullString, binread};
use int_enum::IntEnum;

#[binread]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token(pub u32);

impl Token {
    pub fn index(&self) -> u32 {
        self.0 & 0x00FFFFFF
    }

    fn kind_raw(&self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn kind(&self) -> TokenKind {
        TokenKind::try_from(self.kind_raw()).unwrap_or(TokenKind::Unknown)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_tuple("Token");

        if self.kind() == TokenKind::Unknown {
            #[derive(Debug)]
            #[allow(unused)]
            struct Unknown(u8);

            s.field(&Unknown(self.kind_raw()));
        } else {
            s.field(&self.kind());
        }

        s.field(&self.index()).finish()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum TokenKind {
    Unknown = 0xFF,

    UserString = 0x70,

    Assembly = 0x20,
    AssemblyOS = 0x22,
    AssemblyProcessor = 0x21,
    AssemblyRef = 0x23,
    AssemblyRefOS = 0x25,
    AssemblyRefProcessor = 0x24,
    ClassLayout = 0x0F,
    Constant = 0x0B,
    CustomAttribute = 0x0C,
    DeclSecurity = 0x0E,
    EventMap = 0x12,
    Event = 0x14,
    ExportedType = 0x27,
    Field = 0x04,
    FieldLayout = 0x10,
    FieldMarshal = 0x0D,
    FieldRVA = 0x1D,
    File = 0x26,
    GenericParam = 0x2A,
    GenericParamConstraint = 0x2C,
    ImplMap = 0x1C,
    InterfaceImpl = 0x09,
    ManifestResource = 0x28,
    MemberRef = 0x0A,
    MethodDef = 0x06,
    MethodImpl = 0x19,
    MethodSemantics = 0x18,
    MethodSpec = 0x2B,
    Module = 0x00,
    ModuleRef = 0x1A,
    NestedClass = 0x29,
    Param = 0x08,
    Property = 0x17,
    PropertyMap = 0x15,
    StandAloneSig = 0x11,
    TypeDef = 0x02,
    TypeRef = 0x01,
    TypeSpec = 0x1B,
}

#[binread]
#[derive(Debug)]
pub struct PhysicalMetadata {
    #[br(temp, assert(magic == 0x424A5342))]
    pub magic: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub _reserved: u32,

    #[br(temp)]
    version_length: u32,
    #[br(count = version_length, map = |s: Vec<u8>| String::from_utf8_lossy(&s).trim_end_matches('\0').to_string())]
    pub version: String,

    #[br(align_before = 4)]
    /// Reserved, always 0
    pub flags: u16,

    #[br(temp)]
    stream_count: u16,

    #[br(count = stream_count)]
    pub streams: Vec<StreamHeader>,
}

#[binread]
#[derive(Debug)]
pub struct StreamHeader {
    pub offset: u32,
    pub size: u32,

    #[br(align_after = 4, temp)]
    cname: NullString,

    #[br(calc(cname.to_string()))]
    pub name: String,
}

#[binread]
#[derive(Debug)]
pub struct LogicalMetadataTables {
    pub reserved: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub heap_sizes: u8,
    pub reserved2: u8,
    pub valid: u64,
    pub sorted: u64,

    #[br(calc(valid.count_ones() as usize), temp)]
    pub n: usize,

    #[br(count = n)]
    pub rows_per_table: Vec<u32>,
}

#[binread]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

#[binread]
#[derive(Debug)]
pub struct StringIndex(pub u16);

#[binread]
#[derive(Debug)]
pub struct GuidIndex(pub u16);
