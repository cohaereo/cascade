use binrw::binread;

#[binread]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Opcode {
    #[br(magic(0x00u8))] Nop,
    #[br(magic(0x02u8))] LdArg0,
    #[br(magic(0x03u8))] LdArg1,
    #[br(magic(0x04u8))] LdArg2,
    #[br(magic(0x05u8))] LdArg3,
    #[br(magic(0x28u8))] Call(u32),
    #[br(magic(0x2Au8))] Ret,
    #[br(magic(0x72u8))] LdStr(u32),
    #[br(magic(0x7Du8))] SetFld(u32),
}
