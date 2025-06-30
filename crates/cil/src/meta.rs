use std::fmt::Display;

use binrw::{NullString, binread};

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
