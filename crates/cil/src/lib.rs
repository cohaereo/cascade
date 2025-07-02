pub mod error;
pub mod header;
pub mod image;
pub mod meta;
pub mod opcodes;
pub mod signature;
pub mod strings;
pub mod tables;
mod util;

pub use error::Result;

pub use util::ReadExt;
