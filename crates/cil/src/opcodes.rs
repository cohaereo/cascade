use binrw::binread;
use std::fmt::Display;

use crate::meta::Token;

macro_rules! define_opcodes {
    ($(
        $index:literal => $name:ident ($($fname:ident : $ftype:ident),*) $asmname:expr
    ),*) => {
        #[binread]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[rustfmt::skip]
        #[allow(non_camel_case_types)]
        pub enum Opcode {
            $(
                #[br(magic($index))]
                $name
                    {
                        $(
                            $fname: $ftype,
                        )*
                    },
            )*

        }

        impl Opcode {
            pub fn asm_name(&self) -> &'static str {
                match self {
                    $(Self::$name { .. } => $asmname,)*
                }
            }
        }

        impl Display for Opcode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.asm_name())?;

                match self {
                    $(Self::$name { $($fname,)* } => {
                        $(
                            write!(f, " {:?}", $fname)?;
                        )*
                    })*
                }

                Ok(())
            }
        }
    };
}

define_opcodes! {
    0x00_u8 => Nop() "nop",
    0x02_u8 => LdArg_0() "ldarg.0",
    0x03_u8 => LdArg_1() "ldarg.1",
    0x04_u8 => LdArg_2() "ldarg.2",
    0x05_u8 => LdArg_3() "ldarg.3",

    0x06_u8 => LdLoc_0() "ldloc.0",
    0x07_u8 => LdLoc_1() "ldloc.1",
    0x08_u8 => LdLoc_2() "ldloc.2",
    0x09_u8 => LdLoc_3() "ldloc.3",

    0x0A_u8 => StLoc_0() "stloc.0",
    0x0B_u8 => StLoc_1() "stloc.1",
    0x0C_u8 => StLoc_2() "stloc.2",
    0x0D_u8 => StLoc_3() "stloc.3",

    0x12_u8 => StLoc_S(index: u8) "stloc.s",

    0x15_u8 => LdcI4_M1() "ldc.i4.m1",
    0x16_u8 => LdcI4_0() "ldc.i4.0",
    0x17_u8 => LdcI4_1() "ldc.i4.1",
    0x18_u8 => LdcI4_2() "ldc.i4.2",
    0x19_u8 => LdcI4_3() "ldc.i4.3",
    0x1A_u8 => LdcI4_4() "ldc.i4.4",
    0x1B_u8 => LdcI4_5() "ldc.i4.5",
    0x1C_u8 => LdcI4_6() "ldc.i4.6",
    0x1D_u8 => LdcI4_7() "ldc.i4.7",
    0x1E_u8 => LdcI4_8() "ldc.i4.8",

    0x28_u8 => Call(method: Token) "call",
    0x2A_u8 => Ret() "ret",
    0x2B_u8 => Br_S(offset: i8) "br.s",

    0x58_u8 => Add() "add",
    0x59_u8 => Sub() "sub",
    0x5A_u8 => Mul() "mul",
    0x5B_u8 => Div() "div",
    0x5C_u8 => DivUnsigned() "div.un",
    0x5D_u8 => Rem() "rem",
    0x5E_u8 => RemUnsigned() "rem.un",
    0x5F_u8 => And() "and",
    0x60_u8 => Or() "or",
    0x61_u8 => Xor() "xor",
    0x62_u8 => Shl() "shl",
    0x63_u8 => Shr() "shr",
    0x64_u8 => ShrUnsigned() "shr.un",
    0x65_u8 => Neg() "neg",
    0x66_u8 => Not() "not",


    0x72_u8 => LdStr(string: Token) "ldstr",
    0x7D_u8 => SetFld(field: Token) "stfld"
}
