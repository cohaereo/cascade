use std::fmt::Debug;

use binrw::binread;
use int_enum::IntEnum;

use crate::{bitfield, meta::StringIndex, strings::StringHeap};

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct Field {
    pub flags: FieldAttributes,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    pub signature_blob_index: u16,
}

bitfield! {
    pub struct FieldAttributes : u16 {
        enum access: MemberAccess @ 0x0007 >> 0,
        flag is_static: bool @ 0x0010,
        flag is_init_only: bool @ 0x0020,
        flag is_literal: bool @ 0x0040,
        flag is_not_serialized: bool @ 0x0080,
        flag is_special_name: bool @ 0x0200,
        flag is_pinvoke_impl: bool @ 0x2000,
        flag is_runtime_special_name: bool @ 0x0400,
        flag has_field_marshal: bool @ 0x1000,
        flag has_default: bool @ 0x8000,
        flag has_field_rva: bool @ 0x0100
    }
}

#[repr(u16)]
#[derive(Debug, IntEnum)]
pub enum MemberAccess {
    CompilerControlled = 0,
    Private = 1,
    FamilyAndAssembly = 2,
    /// Also known as "internal"
    Assembly = 3,
    /// Also known as "protected"
    Family = 4,
    FamilyOrAssembly = 5,
    Public = 6,
}

#[binread]
#[derive(Debug)]
pub struct FieldRva {
    pub rva: u32,
    pub field_index: u16,
}
