use std::io::{Read, Seek};
use binrw::{binread, BinRead, BinResult, Endian};
use bitflags::bitflags;

#[binread]
#[derive(Debug)]
pub struct RvaSize {
    pub rva: u32,
    pub size: u32,
}

#[binread]
#[derive(Debug)]
pub struct CliHeader {
    pub size: u32,
    pub major_version: u16, // Currently always 2
    pub minor_version: u16,
    pub physical_metadata: RvaSize,
    pub flags: RuntimeFlags,
    pub entry_point_token: u32,
    pub resources: RvaSize,
    pub strong_name_signature: RvaSize,
    pub code_manager_table: RvaSize,
    pub vtable_fixups: RvaSize,
    pub export_address_table_jumps: RvaSize,
    pub managed_native_header: RvaSize,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RuntimeFlags: u32 {
        const ILONLY = 0x00000001;
        const _32BITREQUIRED = 0x00000002;
        const STRONGNAMESIGNED = 0x00000008;
        const NATIVE_ENTRYPOINT = 0x00000010;
        const TRACKDEBUGDATA = 0x00010000;
    }
}

impl BinRead for RuntimeFlags {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let value: u32 = BinRead::read_options(reader, endian, args)?;
        Ok(RuntimeFlags::from_bits_truncate(value))
    }
}