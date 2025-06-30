use std::fmt::Debug;

use binrw::binread;

use crate::{meta::StringIndex, strings::StringHeap};

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct MemberRef {
    pub class_index: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
    pub signature_blob_index: u16,
}
