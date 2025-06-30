use std::fmt::Debug;

use binrw::binread;

use crate::{meta::StringIndex, strings::StringHeap};

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct Assembly {
    pub hash_algorithm: AssemblyHashAlgorithm,

    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,

    pub flags: u32,
    pub public_key_blob_index: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub culture: String,
}

#[binread]
#[br(repr(u32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AssemblyHashAlgorithm {
    None = 0x0000,
    MD5 = 0x8003,
    SHA1 = 0x8004,
}

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct AssemblyRef {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,

    pub flags: u32,
    pub public_key_or_token_blob_index: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub culture: String,
    pub hash_value_blob_index: u16,
}
