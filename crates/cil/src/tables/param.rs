use std::fmt::Debug;

use binrw::binread;

use crate::{meta::StringIndex, strings::StringHeap};

#[binread]
#[derive(Debug)]
#[br(import(strings: &StringHeap))]
pub struct Param {
    pub flags: u16,
    pub sequence: u16,
    #[br(try_map = |s: StringIndex| strings.try_get(s))]
    pub name: String,
}
