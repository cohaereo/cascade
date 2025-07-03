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

use binrw::{BinRead, BinReaderExt, binread};
use int_enum::IntEnum;

use crate::{
    bitfield,
    image::CilImage,
    meta::{GuidIndex, StringIndex},
    strings::StringHeap,
    util::PackedU32,
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

#[binread]
#[derive(Debug)]
pub struct InterfaceImpl {
    pub class: u16,
    pub interface: u16,
}

#[binread]
#[derive(Debug)]
pub struct Constant {
    pub kind: u8,
    #[br(temp)]
    _pad1: u8,
    pub parent: u16,
    pub value_blob_index: u16,
}

#[binread]
#[derive(Debug)]
pub struct DeclSecurity {
    pub action: u16,
    pub parent: u16,
    pub permission_set_blob_index: u16,
}

#[binread]
#[derive(Debug)]
pub struct ClassLayout {
    pub packing_size: u16,
    pub class_size: u32,
    pub parent: u16,
}

#[binread]
#[derive(Debug)]
pub struct FieldLayout {
    pub offset: u32,
    pub field: u16,
}

#[binread]
#[derive(Debug)]
pub struct PropertyMap {
    pub parent: u16,
    pub property_list: u16,
}

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct Property {
    pub flags: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    pub type_blob_index: u16,
}

#[binread]
#[derive(Debug)]
pub struct MethodSemantics {
    pub semantics: u16,
    pub method: u16,
    pub association: u16,
}

#[binread]
#[derive(Debug)]
pub struct MethodImpl {
    pub class: u16,
    pub method_body: u16,
    pub method_declaration: u16,
}

#[binread]
#[derive(Debug)]
pub struct TypeSpec {
    pub signature_blob_index: u16,
}

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct ImplMap {
    pub mapping_flags: u16,
    pub member_forwarded: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub import_name: String,
    pub import_scope: u16,
}

#[binread]
#[derive(Debug)]
pub struct NestedClass {
    pub nested_class: u16,
    pub enclosing_class: u16,
}

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct GenericParam {
    pub number: u16,
    pub flags: u16,
    pub owner: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
}

#[binread]
#[derive(Debug)]
pub struct MethodSpec {
    pub method: u16,
    pub instantiation_blob_index: u16,
}

#[binread]
#[derive(Debug)]
pub struct GenericParamConstraint {
    pub owner: u16,
    pub constraint: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeDefOrRef {
    TypeDef(u32),
    TypeRef(u32),
    TypeSpec(u32),
}

impl TypeDefOrRef {
    pub fn name_with_namespace(&self, image: &CilImage) -> Option<String> {
        let s = match self {
            TypeDefOrRef::TypeDef(index) => image
                .type_defs
                .get(*index as usize - 1)
                .map(|td| format!("{}.{}", td.type_namespace, td.type_name)),
            TypeDefOrRef::TypeRef(index) => image
                .type_refs
                .get(*index as usize - 1)
                .map(|tr| format!("{}.{}", tr.namespace, tr.name)),
            TypeDefOrRef::TypeSpec(_) => None,
        };

        if let Some(s) = s.clone()
            && s.starts_with('.')
        {
            Some(s[1..].to_string())
        } else {
            s
        }
    }
}

impl BinRead for TypeDefOrRef {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let v: PackedU32 = reader.read_le()?;
        let index = v.0 >> 2;
        match v.0 & 0b11 {
            0 => Ok(TypeDefOrRef::TypeDef(index)),
            1 => Ok(TypeDefOrRef::TypeRef(index)),
            2 => Ok(TypeDefOrRef::TypeSpec(index)),
            _ => Err(binrw::Error::NoVariantMatch {
                pos: reader.stream_position()?,
            }),
        }
    }
}
