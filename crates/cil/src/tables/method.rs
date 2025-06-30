use std::fmt::Debug;

use binrw::binread;
use int_enum::IntEnum;

use crate::{bitfield, meta::StringIndex, strings::StringHeap, tables::MemberAccess};

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct Method {
    pub rva: u32,
    pub impl_flags: MethodImplAttributes,
    pub flags: MethodAttributes,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    pub signature_blob_index: u16,
    pub param_list: u16,
}

bitfield! {
    pub struct MethodImplAttributes : u16 {
        enum code_type: CodeType @ 0x0003 >> 0,
        enum unmanaged: Unmanaged @ 0x0004 >> 2,
        flag is_forward_ref: bool @ 0x0010,
        flag preserve_sig: bool @ 0x0080,
        flag is_internal_call: bool @ 0x1000,
        flag is_synchronized: bool @ 0x0020,
        flag no_inlining: bool @ 0x0008,
        flag no_optimization: bool @ 0x0040,
    }
}

#[repr(u16)]
#[derive(Debug, IntEnum)]
pub enum CodeType {
    IL = 0x0000,
    Native = 0x0001,
    OptIL = 0x0002,
    Runtime = 0x0003,
}

#[repr(u16)]
#[derive(Debug, IntEnum)]
pub enum Unmanaged {
    Managed = 0,
    Unmanaged = 1,
}

bitfield! {
    pub struct MethodAttributes : u16 {
        enum access: MemberAccess @ 0x0007 >> 0,
        flag is_static: bool @ 0x0010,
        flag is_final: bool @ 0x0020,
        flag is_virtual: bool @ 0x0040,
        flag hide_by_sig: bool @ 0x0080,
        enum vtable_layout: VtableLayout @ 0x0100 >> 8,
        flag is_strict: bool @ 0x0200,
        flag is_abstract: bool @ 0x0400,
        flag is_special_name: bool @ 0x0800,
        flag is_pinvoke_impl: bool @ 0x2000,
        flag is_unmanaged_export: bool @ 0x0008,
        flag is_runtime_special_name: bool @ 0x1000,
        flag has_security: bool @ 0x4000,
        flag has_default: bool @ 0x8000,
    }
}

#[repr(u16)]
#[derive(Debug, IntEnum)]
pub enum VtableLayout {
    ReuseSlot = 0,
    NewSlot = 1,
}
