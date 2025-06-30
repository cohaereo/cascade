use std::fmt::Debug;

use binrw::binread;

#[binread]
#[derive(Debug)]
pub struct CustomAttribute {
    pub parent_index: u16,
    pub type_index: u16,
    pub value_blob_index: u16,
}
