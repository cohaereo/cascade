pub mod assembly;
pub mod attribute;
pub mod field;
pub mod member;
pub mod method;
pub mod param;

pub use assembly::*;
pub use attribute::*;
pub use field::*;
pub use member::*;
pub use method::*;
pub use param::*;

use std::fmt::Debug;

use binrw::binread;
use int_enum::IntEnum;

use crate::{
    bitfield,
    meta::{GuidIndex, StringIndex},
    strings::StringHeap,
};

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct Module {
    pub generation: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    pub mvid: GuidIndex,
    pub enc_id: GuidIndex,
    pub enc_base_id: GuidIndex,
}

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct TypeRef {
    pub resolution_scope: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub namespace: String,
}

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct TypeDef {
    pub flags: TypeAttributes,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub type_name: String,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub type_namespace: String,
    pub extends: u16,
    pub field_list: u16,
    pub method_list: u16,
}

bitfield! {
    pub struct TypeAttributes : u32 {
        enum visibility: Visibility @ 0x00000007 >> 0,
        enum layout: Layout @ 0x00000018 >> 3,
        enum class_semantics: ClassSemantics @ 0x00000020 >> 5,
        flag is_abstract: bool @ 0x00000080,
        flag is_sealed: bool @ 0x00000100,
        flag is_special_name: bool @ 0x00000400,
        flag is_import: bool @ 0x00001000,
        flag is_serializable: bool @ 0x00002000,
        enum native_string_format: NativeStringFormat @ 0x00030000 >> 16,
        flag is_rt_special_name: bool @ 0x00000800,
        flag has_security: bool @ 0x00040000,
        flag is_type_forwarder: bool @ 0x00200000,
    }
}

#[repr(u32)]
#[derive(Debug, IntEnum)]
pub enum Visibility {
    NotPublic = 0,
    Public = 1,
    NestedPublic = 2,
    NestedPrivate = 3,
    NestedFamily = 4,
    NestedAssembly = 5,
    NestedFamilyAndAssembly = 6,
    NestedFamilyOrAssembly = 7,
}

#[repr(u32)]
#[derive(Debug, IntEnum)]
pub enum Layout {
    Auto = 0,
    Sequential = 1,
    Explicit = 2,
}

#[repr(u32)]
#[derive(Debug, IntEnum)]
pub enum NativeStringFormat {
    Ansi = 0,
    Unicode = 1,
    Auto = 2,
    Custom = 3,
}

#[repr(u32)]
#[derive(Debug, IntEnum)]
pub enum ClassSemantics {
    Class,
    Interface,
}

#[binread]
#[derive(Debug)]
pub struct StandAloneSig {
    pub signature_blob_index: u16,
}
