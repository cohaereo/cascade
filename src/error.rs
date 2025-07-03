use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Decompiler format write error {0}")]
    FmtError(#[from] std::fmt::Error),

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Unimplemented opcode: {0:?}")]
    UnimplementedOpcode(crate::opcodes::Opcode),
}

pub type Result<T> = std::result::Result<T, Error>;
