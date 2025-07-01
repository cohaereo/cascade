use binrw::BinReaderExt;
use binrw::binread;
use std::fmt::Display;

use crate::meta::Token;

macro_rules! define_opcodes {
    ($(
        $index:literal => $name:ident ($($fname:ident : $ftype:ident),*) $asmname:expr
    ),*) => {
        #[derive(Debug, Clone, PartialEq)]
        #[rustfmt::skip]
        #[allow(non_camel_case_types)]
        pub enum Opcode {
            $(
                $name
                    {
                        $(
                            $fname: $ftype,
                        )*
                    },
            )*
            Switch { targets: Vec<i32> },
        }

        impl Opcode {
            pub fn asm_name(&self) -> &'static str {
                match self {
                    $(Self::$name { .. } => $asmname,)*
                    Self::Switch { .. } => "switch",
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
                    })*,
                    Self::Switch { targets } => {
                        write!(f, " [")?;
                        for (i, target) in targets.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}", target)?;
                        }
                        write!(f, "]")?;
                    }
                }

                Ok(())
            }
        }

        impl binrw::BinRead for Opcode {
            type Args<'a> = ();

            fn read_options<R: std::io::Read + std::io::Seek>(reader: &mut R, endian: binrw::Endian, args: Self::Args<'_>) -> binrw::BinResult<Self> {
                let mut index: u16 = reader.read_type::<u8>(endian)? as u16;
                if index == 0xFE {
                    reader.seek(std::io::SeekFrom::Current(-1))?; // Rewind one byte
                    index = reader.read_be::<u16>()?;
                }

                match index {
                    $(
                        $index => Ok(Self::$name {
                            $(
                                $fname: binrw::BinRead::read_options(reader, endian, args)?,
                            )*
                        }),
                    )*
                    0x45 => {
                        let target_count: u32 = reader.read_type(endian)?;
                        Ok(Self::Switch { targets: reader.read_type_args::<Vec<i32>>(endian, binrw::args! { count: target_count as usize })? })
                    }
                    _ => Err(binrw::Error::AssertFail {
                        pos: reader.stream_position().unwrap_or(0),
                        message: format!("Unknown opcode index 0x{index:04X}"),
                    }),
                }
            }
        }
    };
}

define_opcodes! {
    0x00 => Nop() "nop",
    0x01 => Break() "break",
    0x02 => LdArg_0() "ldarg.0",
    0x03 => LdArg_1() "ldarg.1",
    0x04 => LdArg_2() "ldarg.2",
    0x05 => LdArg_3() "ldarg.3",

    0x06 => LdLoc_0() "ldloc.0",
    0x07 => LdLoc_1() "ldloc.1",
    0x08 => LdLoc_2() "ldloc.2",
    0x09 => LdLoc_3() "ldloc.3",

    0x0A => StLoc_0() "stloc.0",
    0x0B => StLoc_1() "stloc.1",
    0x0C => StLoc_2() "stloc.2",
    0x0D => StLoc_3() "stloc.3",

    0x0E => LdArg_S(index: u8) "ldarg.s",
    0x0F => LdArgA_S(index: u8) "ldarga.s",
    0x10 => StArg_S(index: u8) "starg.s",
    0x11 => LdLoc_S(index: u8) "ldloc.s",
    0x12 => LdLocA_S(index: u8) "ldloca.s",
    0x13 => StLoc_S(index: u8) "stloc.s",

    0x14 => LdNull() "ldnull",
    0x15 => Ldc_I4_M1() "ldc.i4.m1",
    0x16 => Ldc_I4_0() "ldc.i4.0",
    0x17 => Ldc_I4_1() "ldc.i4.1",
    0x18 => Ldc_I4_2() "ldc.i4.2",
    0x19 => Ldc_I4_3() "ldc.i4.3",
    0x1A => Ldc_I4_4() "ldc.i4.4",
    0x1B => Ldc_I4_5() "ldc.i4.5",
    0x1C => Ldc_I4_6() "ldc.i4.6",
    0x1D => Ldc_I4_7() "ldc.i4.7",
    0x1E => Ldc_I4_8() "ldc.i4.8",
    0x1F => Ldc_I4_S(value: i8) "ldc.i4.s",
    0x20 => Ldc_I4(value: i32) "ldc.i4",
    0x21 => Ldc_I8(value: i64) "ldc.i8",
    0x22 => Ldc_R4(value: f32) "ldc.r4",
    0x23 => Ldc_R8(value: f64) "ldc.r8",

    0x25 => Dup() "dup",
    0x26 => Pop() "pop",

    0x27 => Jmp(method: Token) "jmp",
    0x28 => Call(method: Token) "call",
    0x29 => CallInd(callsitedescr: Token) "calli",
    0x2A => Ret() "ret",
    0x2B => Br_S(offset: i8) "br.s",
    0x2C => Br_False_S(offset: i8) "brfalse.s",
    0x2D => Br_True_S(offset: i8) "brtrue.s",
    0x2E => Beq_S(offset: i8) "beq.s",
    0x2F => Bge_S(offset: i8) "bge.s",
    0x30 => Bgt_S(offset: i8) "bgt.s",
    0x31 => Ble_S(offset: i8) "ble.s",
    0x32 => Blt_S(offset: i8) "blt.s",
    0x33 => Bne_Un_S(offset: i8) "bne.un.s",
    0x34 => Bge_Un_S(offset: i8) "bge.un.s",
    0x35 => Bgt_Un_S(offset: i8) "bgt.un.s",
    0x36 => Ble_Un_S(offset: i8) "ble.un.s",
    0x37 => Blt_Un_S(offset: i8) "blt.un.s",
    0x38 => Br(offset: i32) "br",
    0x39 => Br_False(offset: i32) "brfalse",
    0x3A => Br_True(offset: i32) "brtrue",
    0x3B => Beq(offset: i32) "beq",
    0x3C => Bge(offset: i32) "bge",
    0x3D => Bgt(offset: i32) "bgt",
    0x3E => Ble(offset: i32) "ble",
    0x3F => Blt(offset: i32) "blt",
    0x40 => Bne_Un(offset: i32) "bne.un",
    0x41 => Bge_Un(offset: i32) "bge.un",
    0x42 => Bgt_Un(offset: i32) "bgt.un",
    0x43 => Ble_Un(offset: i32) "ble.un",
    0x44 => Blt_Un(offset: i32) "blt.un",
    // Defined in macro
    // 0x45 => Switch(targets: Vec<i32>) "switch",

    0x46 => LdInd_I1() "ldind.i1",
    0x47 => LdInd_U1() "ldind.u1",
    0x48 => LdInd_I2() "ldind.i2",
    0x49 => LdInd_U2() "ldind.u2",
    0x4A => LdInd_I4() "ldind.i4",
    0x4B => LdInd_U4() "ldind.u4",
    0x4C => LdInd_I8() "ldind.i8",
    0x4D => LdInd_I() "ldind.i",
    0x4E => LdInd_R4() "ldind.r4",
    0x4F => LdInd_R8() "ldind.r8",
    0x50 => LdInd_Ref() "ldind.ref",
    0x51 => StInd_Ref() "stind.ref",
    0x52 => StInd_I1() "stind.i1",
    0x53 => StInd_I2() "stind.i2",
    0x54 => StInd_I4() "stind.i4",
    0x55 => StInd_I8() "stind.i8",
    0x56 => StInd_R4() "stind.r4",
    0x57 => StInd_R8() "stind.r8",

    0x58 => Add() "add",
    0x59 => Sub() "sub",
    0x5A => Mul() "mul",
    0x5B => Div() "div",
    0x5C => DivUnsigned() "div.un",
    0x5D => Rem() "rem",
    0x5E => RemUnsigned() "rem.un",
    0x5F => And() "and",
    0x60 => Or() "or",
    0x61 => Xor() "xor",
    0x62 => Shl() "shl",
    0x63 => Shr() "shr",
    0x64 => ShrUnsigned() "shr.un",
    0x65 => Neg() "neg",
    0x66 => Not() "not",

    0x67 => Conv_I1() "conv.i1",
    0x68 => Conv_I2() "conv.i2",
    0x69 => Conv_I4() "conv.i4",
    0x6A => Conv_I8() "conv.i8",
    0x6B => Conv_R4() "conv.r4",
    0x6C => Conv_R8() "conv.r8",
    0x6D => Conv_U4() "conv.u4",
    0x6E => Conv_U8() "conv.u8",

    0x6F => CallVirt(method: Token) "callvirt",

    0x70 => CpObj(typeref: Token) "cpobj",
    0x71 => LdObj(typeref: Token) "ldobj",
    0x72 => LdStr(string: Token) "ldstr",
    0x73 => NewObj(ctor: Token) "newobj",
    0x74 => CastClass(typeref: Token) "castclass",
    0x75 => IsInst(typeref: Token) "isinst",
    0x76 => Conv_R_Un() "conv.r.un",
    0x79 => Unbox(typeref: Token) "unbox",
    0x7A => Throw() "throw",
    0x7B => LdFld(field: Token) "ldfld",
    0x7C => LdFlda(field: Token) "ldflda",
    0x7D => SetFld(field: Token) "stfld",
    0x7E => GetFld(field: Token) "ldfld",
    0x7F => GetFlda(field: Token) "ldflda",
    0x80 => StsFld(field: Token) "stsfld",
    0x81 => StObj(typeref: Token) "stobj",
    0x82 => Conv_Ovf_I1_Unsigned() "conv.ovf.i1.un",
    0x83 => Conv_Ovf_I2_Unsigned() "conv.ovf.i2.un",
    0x84 => Conv_Ovf_I4_Unsigned() "conv.ovf.i4.un",
    0x85 => Conv_Ovf_I8_Unsigned() "conv.ovf.i8.un",
    0x86 => Conv_Ovf_U1_Unsigned() "conv.ovf.u1.un",
    0x87 => Conv_Ovf_U2_Unsigned() "conv.ovf.u2.un",
    0x88 => Conv_Ovf_U4_Unsigned() "conv.ovf.u4.un",
    0x89 => Conv_Ovf_U8_Unsigned() "conv.ovf.u8.un",
    0x8A => Conv_Ovf_I_Unsigned() "conv.ovf.i.un",
    0x8B => Conv_Ovf_U_Unsigned() "conv.ovf.u.un",
    0x8C => Box(typeref: Token) "box",
    0x8D => NewArr(typeref: Token) "newarr",
    0x8E => LdLen() "ldlen",
    0x8F => LdElema(class: Token) "ldelema",
    0x90 => LdElem_I1() "ldelem.i1",
    0x91 => LdElem_U1() "ldelem.u1",
    0x92 => LdElem_I2() "ldelem.i2",
    0x93 => LdElem_U2() "ldelem.u2",
    0x94 => LdElem_I4() "ldelem.i4",
    0x95 => LdElem_U4() "ldelem.u4",
    0x96 => LdElem_I8() "ldelem.i8",
    0x97 => LdElem_I() "ldelem.i",
    0x98 => LdElem_R4() "ldelem.r4",
    0x99 => LdElem_R8() "ldelem.r8",
    0x9A => LdElem_Ref() "ldelem.ref",
    0x9B => StElem_I() "stelem.i",
    0x9C => StElem_I1() "stelem.i1",
    0x9D => StElem_I2() "stelem.i2",
    0x9E => StElem_I4() "stelem.i4",
    0x9F => StElem_I8() "stelem.i8",
    0xA0 => StElem_R4() "stelem.r4",
    0xA1 => StElem_R8() "stelem.r8",
    0xA2 => StElem_Ref() "stelem.ref",
    0xA3 => LdElem_Any(typeref: Token) "ldelem.any",
    0xA4 => StElem_Any(typeref: Token) "stelem.any",
    0xA5 => Unbox_Any(typeref: Token) "unbox.any",
    0xB3 => Conv_Ovf_I1() "conv.ovf.i1",
    0xB4 => Conv_Ovf_U1() "conv.ovf.u1",
    0xB5 => Conv_Ovf_I2() "conv.ovf.i2",
    0xB6 => Conv_Ovf_U2() "conv.ovf.u2",
    0xB7 => Conv_Ovf_I4() "conv.ovf.i4",
    0xB8 => Conv_Ovf_U4() "conv.ovf.u4",
    0xB9 => Conv_Ovf_I8() "conv.ovf.i8",
    0xBA => Conv_Ovf_U8() "conv.ovf.u8",
    0xC2 => RefAnyVal(class: Token) "refanyval",
    0xC3 => CkFinite(typeref: Token) "ckfinite",
    0xC6 => MkRefAny(typeref: Token) "mkrefany",
    0xD0 => LdToken(token: Token) "ldtoken",
    0xD1 => Conv_U2() "conv.u2",
    0xD2 => Conv_U1() "conv.u1",
    0xD3 => Conv_I() "conv.i",
    0xD4 => Conv_Ovf_I() "conv.ovf.i",
    0xD5 => Conv_Ovf_U() "conv.ovf.u",
    0xD6 => Add_Ovf() "add.ovf",
    0xD7 => Add_Ovf_Unsigned() "add.ovf.un",
    0xD8 => Mul_Ovf() "mul.ovf",
    0xD9 => Mul_Ovf_Unsigned() "mul.ovf.un",
    0xDA => Sub_Ovf() "sub.ovf",
    0xDB => Sub_Ovf_Unsigned() "sub.ovf.un",
    0xDC => EndFaultOrFinally() "endfault/endfinally",
    0xDD => Leave(offset: i32) "leave",
    0xDE => Leave_S(offset: i8) "leave.s",
    0xDF => StInd_I() "stind.i",
    0xE0 => Conv_U() "conv.u",

    0xFE_00 => ArgList() "arglist",
    0xFE_01 => Ceq() "ceq",
    0xFE_02 => Cgt() "cgt",
    0xFE_03 => Cgt_Un() "cgt.un",
    0xFE_04 => Clt() "clt",
    0xFE_05 => Clt_Un() "clt.un",
    0xFE_06 => LdFtn(method: Token) "ldftn",
    0xFE_07 => LdVirtFtn(method: Token) "ldvirtftn",
    0xFE_09 => LdArg(index: u16) "ldarg",
    0xFE_0A => LdArgA(index: u16) "ldarga",
    0xFE_0B => StArg(index: u16) "starg",
    0xFE_0C => LdLoc(index: u16) "ldloc",
    0xFE_0D => LdLocA(index: u16) "ldloca",
    0xFE_0E => StLoc(index: u16) "stloc",
    0xFE_0F => LocAlloc(size: u32) "localloc",
    0xFE_11 => EndFilter() "endfilter",
    0xFE_12 => Unaligned() "unaligned.",
    0xFE_13 => Volatile() "volatile.",
    0xFE_14 => Tail() "tail.",
    0xFE_15 => InitObj(type_tok: Token) "initobj",
    0xFE_16 => Constrained(this_type: Token) "constrained."
}

// 0x00 	nop 	Do nothing (No operation). 	Base instruction
// 0x01 	break 	Inform a debugger that a breakpoint has been reached. 	Base instruction
// 0x02 	ldarg.0 	Load argument 0 onto the stack. 	Base instruction
// 0x03 	ldarg.1 	Load argument 1 onto the stack. 	Base instruction
// 0x04 	ldarg.2 	Load argument 2 onto the stack. 	Base instruction
// 0x05 	ldarg.3 	Load argument 3 onto the stack. 	Base instruction
// 0x06 	ldloc.0 	Load local variable 0 onto stack. 	Base instruction
// 0x07 	ldloc.1 	Load local variable 1 onto stack. 	Base instruction
// 0x08 	ldloc.2 	Load local variable 2 onto stack. 	Base instruction
// 0x09 	ldloc.3 	Load local variable 3 onto stack. 	Base instruction
// 0x0A 	stloc.0 	Pop a value from stack into local variable 0. 	Base instruction
// 0x0B 	stloc.1 	Pop a value from stack into local variable 1. 	Base instruction
// 0x0C 	stloc.2 	Pop a value from stack into local variable 2. 	Base instruction
// 0x0D 	stloc.3 	Pop a value from stack into local variable 3. 	Base instruction
// 0x0E 	ldarg.s <uint8 (num)> 	Load argument numbered num onto the stack, short form. 	Base instruction
// 0x0F 	ldarga.s <uint8 (argNum)> 	Fetch the address of argument argNum, short form. 	Base instruction
// 0x10 	starg.s <uint8 (num)> 	Store value to the argument numbered num, short form. 	Base instruction
// 0x11 	ldloc.s <uint8 (indx)> 	Load local variable of index indx onto stack, short form. 	Base instruction
// 0x12 	ldloca.s <uint8 (indx)> 	Load address of local variable with index indx, short form. 	Base instruction
// 0x13 	stloc.s <uint8 (indx)> 	Pop a value from stack into local variable indx, short form. 	Base instruction
// 0x14 	ldnull 	Push a null reference on the stack. 	Base instruction
// 0x15 	ldc.i4.m1 	Push -1 onto the stack as int32. 	Base instruction
// 0x15 	ldc.i4.M1 	Push -1 onto the stack as int32 (alias for ldc.i4.m1). 	Base instruction
// 0x16 	ldc.i4.0 	Push 0 onto the stack as int32. 	Base instruction
// 0x17 	ldc.i4.1 	Push 1 onto the stack as int32. 	Base instruction
// 0x18 	ldc.i4.2 	Push 2 onto the stack as int32. 	Base instruction
// 0x19 	ldc.i4.3 	Push 3 onto the stack as int32. 	Base instruction
// 0x1A 	ldc.i4.4 	Push 4 onto the stack as int32. 	Base instruction
// 0x1B 	ldc.i4.5 	Push 5 onto the stack as int32. 	Base instruction
// 0x1C 	ldc.i4.6 	Push 6 onto the stack as int32. 	Base instruction
// 0x1D 	ldc.i4.7 	Push 7 onto the stack as int32. 	Base instruction
// 0x1E 	ldc.i4.8 	Push 8 onto the stack as int32. 	Base instruction
// 0x1F 	ldc.i4.s <int8 (num)> 	Push num onto the stack as int32, short form. 	Base instruction
// 0x20 	ldc.i4 <int32 (num)> 	Push num of type int32 onto the stack as int32. 	Base instruction
// 0x21 	ldc.i8 <int64 (num)> 	Push num of type int64 onto the stack as int64. 	Base instruction
// 0x22 	ldc.r4 <float32 (num)> 	Push num of type float32 onto the stack as F. 	Base instruction
// 0x23 	ldc.r8 <float64 (num)> 	Push num of type float64 onto the stack as F. 	Base instruction
// 0x25 	dup 	Duplicate the value on the top of the stack. 	Base instruction
// 0x26 	pop 	Pop value from the stack. 	Base instruction
// 0x27 	jmp <method> 	Exit current method and jump to the specified method. 	Base instruction
// 0x28 	call <method> 	Call method described by method. 	Base instruction
// 0x29 	calli <callsitedescr> 	Call method indicated on the stack with arguments described by callsitedescr. 	Base instruction
// 0x2A 	ret 	Return from method, possibly with a value. 	Base instruction
// 0x2B 	br.s <int8 (target)> 	Branch to target, short form. 	Base instruction
// 0x2C 	brfalse.s <int8 (target)> 	Branch to target if value is zero (false), short form. 	Base instruction
// 0x2C 	brnull.s <int8 (target)> 	Branch to target if value is null (alias for brfalse.s), short form. 	Base instruction
// 0x2C 	brzero.s <int8 (target)> 	Branch to target if value is zero (alias for brfalse.s), short form. 	Base instruction
// 0x2D 	brinst.s <int8 (target)> 	Branch to target if value is a non-null object reference, short form (alias for brtrue.s). 	Base instruction
// 0x2D 	brtrue.s <int8 (target)> 	Branch to target if value is non-zero (true), short form. 	Base instruction
// 0x2E 	beq.s <int8 (target)> 	Branch to target if equal, short form. 	Base instruction
// 0x2F 	bge.s <int8 (target)> 	Branch to target if greater than or equal to, short form. 	Base instruction
// 0x30 	bgt.s <int8 (target)> 	Branch to target if greater than, short form. 	Base instruction
// 0x31 	ble.s <int8 (target)> 	Branch to target if less than or equal to, short form. 	Base instruction
// 0x32 	blt.s <int8 (target)> 	Branch to target if less than, short form. 	Base instruction
// 0x33 	bne.un.s <int8 (target)> 	Branch to target if unequal or unordered, short form. 	Base instruction
// 0x34 	bge.un.s <int8 (target)> 	Branch to target if greater than or equal to (unsigned or unordered), short form. 	Base instruction
// 0x35 	bgt.un.s <int8 (target)> 	Branch to target if greater than (unsigned or unordered), short form. 	Base instruction
// 0x36 	ble.un.s <int8 (target)> 	Branch to target if less than or equal to (unsigned or unordered), short form. 	Base instruction
// 0x37 	blt.un.s <int8 (target)> 	Branch to target if less than (unsigned or unordered), short form. 	Base instruction
// 0x38 	br <int32 (target)> 	Branch to target. 	Base instruction
// 0x39 	brfalse <int32 (target)> 	Branch to target if value is zero (false). 	Base instruction
// 0x39 	brnull <int32 (target)> 	Branch to target if value is null (alias for brfalse). 	Base instruction
// 0x39 	brzero <int32 (target)> 	Branch to target if value is zero (alias for brfalse). 	Base instruction
// 0x3A 	brinst <int32 (target)> 	Branch to target if value is a non-null object reference (alias for brtrue). 	Base instruction
// 0x3A 	brtrue <int32 (target)> 	Branch to target if value is non-zero (true). 	Base instruction
// 0x3B 	beq <int32 (target)> 	Branch to target if equal. 	Base instruction
// 0x3C 	bge <int32 (target)> 	Branch to target if greater than or equal to. 	Base instruction
// 0x3D 	bgt <int32 (target)> 	Branch to target if greater than. 	Base instruction
// 0x3E 	ble <int32 (target)> 	Branch to target if less than or equal to. 	Base instruction
// 0x3F 	blt <int32 (target)> 	Branch to target if less than. 	Base instruction
// 0x40 	bne.un <int32 (target)> 	Branch to target if unequal or unordered. 	Base instruction
// 0x41 	bge.un <int32 (target)> 	Branch to target if greater than or equal to (unsigned or unordered). 	Base instruction
// 0x42 	bgt.un <int32 (target)> 	Branch to target if greater than (unsigned or unordered). 	Base instruction
// 0x43 	ble.un <int32 (target)> 	Branch to target if less than or equal to (unsigned or unordered). 	Base instruction
// 0x44 	blt.un <int32 (target)> 	Branch to target if less than (unsigned or unordered). 	Base instruction
// 0x45 	switch <uint32, int32, int32 (t1..tN)> 	Jump to one of n values. 	Base instruction
// 0x46 	ldind.i1 	Indirect load value of type int8 as int32 on the stack. 	Base instruction
// 0x47 	ldind.u1 	Indirect load value of type unsigned int8 as int32 on the stack. 	Base instruction
// 0x48 	ldind.i2 	Indirect load value of type int16 as int32 on the stack. 	Base instruction
// 0x49 	ldind.u2 	Indirect load value of type unsigned int16 as int32 on the stack. 	Base instruction
// 0x4A 	ldind.i4 	Indirect load value of type int32 as int32 on the stack. 	Base instruction
// 0x4B 	ldind.u4 	Indirect load value of type unsigned int32 as int32 on the stack. 	Base instruction
// 0x4C 	ldind.i8 	Indirect load value of type int64 as int64 on the stack. 	Base instruction
// 0x4C 	ldind.u8 	Indirect load value of type unsigned int64 as int64 on the stack (alias for ldind.i8). 	Base instruction
// 0x4D 	ldind.i 	Indirect load value of type native int as native int on the stack. 	Base instruction
// 0x4E 	ldind.r4 	Indirect load value of type float32 as F on the stack. 	Base instruction
// 0x4F 	ldind.r8 	Indirect load value of type float64 as F on the stack. 	Base instruction
// 0x50 	ldind.ref 	Indirect load value of type object ref as O on the stack. 	Base instruction
// 0x51 	stind.ref 	Store value of type object ref (type O) into memory at address. 	Base instruction
// 0x52 	stind.i1 	Store value of type int8 into memory at address. 	Base instruction
// 0x53 	stind.i2 	Store value of type int16 into memory at address. 	Base instruction
// 0x54 	stind.i4 	Store value of type int32 into memory at address. 	Base instruction
// 0x55 	stind.i8 	Store value of type int64 into memory at address. 	Base instruction
// 0x56 	stind.r4 	Store value of type float32 into memory at address. 	Base instruction
// 0x57 	stind.r8 	Store value of type float64 into memory at address. 	Base instruction
// 0x58 	add 	Add two values, returning a new value. 	Base instruction
// 0x59 	sub 	Subtract value2 from value1, returning a new value. 	Base instruction
// 0x5A 	mul 	Multiply values. 	Base instruction
// 0x5B 	div 	Divide two values to return a quotient or floating-point result. 	Base instruction
// 0x5C 	div.un 	Divide two values, unsigned, returning a quotient. 	Base instruction
// 0x5D 	rem 	Remainder when dividing one value by another. 	Base instruction
// 0x5E 	rem.un 	Remainder when dividing one unsigned value by another. 	Base instruction
// 0x5F 	and 	Bitwise AND of two integral values, returns an integral value. 	Base instruction
// 0x60 	or 	Bitwise OR of two integer values, returns an integer. 	Base instruction
// 0x61 	xor 	Bitwise XOR of integer values, returns an integer. 	Base instruction
// 0x62 	shl 	Shift an integer left (shifting in zeros), return an integer. 	Base instruction
// 0x63 	shr 	Shift an integer right (shift in sign), return an integer. 	Base instruction
// 0x64 	shr.un 	Shift an integer right (shift in zero), return an integer. 	Base instruction
// 0x65 	neg 	Negate value. 	Base instruction
// 0x66 	not 	Bitwise complement. 	Base instruction
// 0x67 	conv.i1 	Convert to int8, pushing int32 on stack. 	Base instruction
// 0x68 	conv.i2 	Convert to int16, pushing int32 on stack. 	Base instruction
// 0x69 	conv.i4 	Convert to int32, pushing int32 on stack. 	Base instruction
// 0x6A 	conv.i8 	Convert to int64, pushing int64 on stack. 	Base instruction
// 0x6B 	conv.r4 	Convert to float32, pushing F on stack. 	Base instruction
// 0x6C 	conv.r8 	Convert to float64, pushing F on stack. 	Base instruction
// 0x6D 	conv.u4 	Convert to unsigned int32, pushing int32 on stack. 	Base instruction
// 0x6E 	conv.u8 	Convert to unsigned int64, pushing int64 on stack. 	Base instruction
// 0x6F 	callvirt <method> 	Call a method associated with an object. 	Object model instruction
// 0x70 	cpobj <typeTok> 	Copy a value type from src to dest. 	Object model instruction
// 0x71 	ldobj <typeTok> 	Copy the value stored at address src to the stack. 	Object model instruction
// 0x72 	ldstr <string> 	Push a string object for the literal string. 	Object model instruction
// 0x73 	newobj <ctor> 	Allocate an uninitialized object or value type and call ctor. 	Object model instruction
// 0x74 	castclass <class> 	Cast obj to class. 	Object model instruction
// 0x75 	isinst <class> 	Test if obj is an instance of class, returning null or an instance of that class or interface. 	Object model instruction
// 0x76 	conv.r.un 	Convert unsigned integer to floating-point, pushing F on stack. 	Base instruction
// 0x79 	unbox <valuetype> 	Extract a value-type from obj, its boxed representation, and push a controlled-mutability managed pointer to it to the top of the stack. 	Object model instruction
// 0x7A 	throw 	Throw an exception. 	Object model instruction
// 0x7B 	ldfld <field> 	Push the value of field of object (or value type) obj, onto the stack. 	Object model instruction
// 0x7C 	ldflda <field> 	Push the address of field of object obj on the stack. 	Object model instruction
// 0x7D 	stfld <field> 	Replace the value of field of the object obj with value. 	Object model instruction
// 0x7E 	ldsfld <field> 	Push the value of the static field on the stack. 	Object model instruction
// 0x7F 	ldsflda <field> 	Push the address of the static field, field, on the stack. 	Object model instruction
// 0x80 	stsfld <field> 	Replace the value of the static field with val. 	Object model instruction
// 0x81 	stobj <typeTok> 	Store a value of type typeTok at an address. 	Object model instruction
// 0x82 	conv.ovf.i1.un 	Convert unsigned to an int8 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0x83 	conv.ovf.i2.un 	Convert unsigned to an int16 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0x84 	conv.ovf.i4.un 	Convert unsigned to an int32 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0x85 	conv.ovf.i8.un 	Convert unsigned to an int64 (on the stack as int64) and throw an exception on overflow. 	Base instruction
// 0x86 	conv.ovf.u1.un 	Convert unsigned to an unsigned int8 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0x87 	conv.ovf.u2.un 	Convert unsigned to an unsigned int16 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0x88 	conv.ovf.u4.un 	Convert unsigned to an unsigned int32 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0x89 	conv.ovf.u8.un 	Convert unsigned to an unsigned int64 (on the stack as int64) and throw an exception on overflow. 	Base instruction
// 0x8A 	conv.ovf.i.un 	Convert unsigned to a native int (on the stack as native int) and throw an exception on overflow. 	Base instruction
// 0x8B 	conv.ovf.u.un 	Convert unsigned to a native unsigned int (on the stack as native int) and throw an exception on overflow. 	Base instruction
// 0x8C 	box <typeTok> 	Convert a boxable value to its boxed form. 	Object model instruction
// 0x8D 	newarr <etype> 	Create a new array with elements of type etype. 	Object model instruction
// 0x8E 	ldlen 	Push the length (of type native unsigned int) of array on the stack. 	Object model instruction
// 0x8F 	ldelema <class> 	Load the address of element at index onto the top of the stack. 	Object model instruction
// 0x90 	ldelem.i1 	Load the element with type int8 at index onto the top of the stack as an int32. 	Object model instruction
// 0x91 	ldelem.u1 	Load the element with type unsigned int8 at index onto the top of the stack as an int32. 	Object model instruction
// 0x92 	ldelem.i2 	Load the element with type int16 at index onto the top of the stack as an int32. 	Object model instruction
// 0x93 	ldelem.u2 	Load the element with type unsigned int16 at index onto the top of the stack as an int32. 	Object model instruction
// 0x94 	ldelem.i4 	Load the element with type int32 at index onto the top of the stack as an int32. 	Object model instruction
// 0x95 	ldelem.u4 	Load the element with type unsigned int32 at index onto the top of the stack as an int32. 	Object model instruction
// 0x96 	ldelem.i8 	Load the element with type int64 at index onto the top of the stack as an int64. 	Object model instruction
// 0x96 	ldelem.u8 	Load the element with type unsigned int64 at index onto the top of the stack as an int64 (alias for ldelem.i8). 	Object model instruction
// 0x97 	ldelem.i 	Load the element with type native int at index onto the top of the stack as a native int. 	Object model instruction
// 0x98 	ldelem.r4 	Load the element with type float32 at index onto the top of the stack as an F. 	Object model instruction
// 0x99 	ldelem.r8 	Load the element with type float64 at index onto the top of the stack as an F. 	Object model instruction
// 0x9A 	ldelem.ref 	Load the element at index onto the top of the stack as an O. The type of the O is the same as the element type of the array pushed on the CIL stack. 	Object model instruction
// 0x9B 	stelem.i 	Replace array element at index with the native int value on the stack. 	Object model instruction
// 0x9C 	stelem.i1 	Replace array element at index with the int8 value on the stack. 	Object model instruction
// 0x9D 	stelem.i2 	Replace array element at index with the int16 value on the stack. 	Object model instruction
// 0x9E 	stelem.i4 	Replace array element at index with the int32 value on the stack. 	Object model instruction
// 0x9F 	stelem.i8 	Replace array element at index with the int64 value on the stack. 	Object model instruction
// 0xA0 	stelem.r4 	Replace array element at index with the float32 value on the stack. 	Object model instruction
// 0xA1 	stelem.r8 	Replace array element at index with the float64 value on the stack. 	Object model instruction
// 0xA2 	stelem.ref 	Replace array element at index with the ref value on the stack. 	Object model instruction
// 0xA3 	ldelem <typeTok> 	Load the element at index onto the top of the stack. 	Object model instruction
// 0xA4 	stelem <typeTok> 	Replace array element at index with the value on the stack. 	Object model instruction
// 0xA5 	unbox.any <typeTok> 	Extract a value-type from obj, its boxed representation, and copy to the top of the stack. 	Object model instruction
// 0xB3 	conv.ovf.i1 	Convert to an int8 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0xB4 	conv.ovf.u1 	Convert to an unsigned int8 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0xB5 	conv.ovf.i2 	Convert to an int16 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0xB6 	conv.ovf.u2 	Convert to an unsigned int16 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0xB7 	conv.ovf.i4 	Convert to an int32 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0xB8 	conv.ovf.u4 	Convert to an unsigned int32 (on the stack as int32) and throw an exception on overflow. 	Base instruction
// 0xB9 	conv.ovf.i8 	Convert to an int64 (on the stack as int64) and throw an exception on overflow. 	Base instruction
// 0xBA 	conv.ovf.u8 	Convert to an unsigned int64 (on the stack as int64) and throw an exception on overflow. 	Base instruction
// 0xC2 	refanyval <type> 	Push the address stored in a typed reference. 	Object model instruction
// 0xC3 	ckfinite 	Throw ArithmeticException if value is not a finite number. 	Base instruction
// 0xC6 	mkrefany <class> 	Push a typed reference to ptr of type class onto the stack. 	Object model instruction
// 0xD0 	ldtoken <token> 	Convert metadata token to its runtime representation. 	Object model instruction
// 0xD1 	conv.u2 	Convert to unsigned int16, pushing int32 on stack. 	Base instruction
// 0xD2 	conv.u1 	Convert to unsigned int8, pushing int32 on stack. 	Base instruction
// 0xD3 	conv.i 	Convert to native int, pushing native int on stack. 	Base instruction
// 0xD4 	conv.ovf.i 	Convert to a native int (on the stack as native int) and throw an exception on overflow. 	Base instruction
// 0xD5 	conv.ovf.u 	Convert to a native unsigned int (on the stack as native int) and throw an exception on overflow. 	Base instruction
// 0xD6 	add.ovf 	Add signed integer values with overflow check. 	Base instruction
// 0xD7 	add.ovf.un 	Add unsigned integer values with overflow check. 	Base instruction
// 0xD8 	mul.ovf 	Multiply signed integer values. Signed result shall fit in same size. 	Base instruction
// 0xD9 	mul.ovf.un 	Multiply unsigned integer values. Unsigned result shall fit in same size. 	Base instruction
// 0xDA 	sub.ovf 	Subtract native int from a native int. Signed result shall fit in same size. 	Base instruction
// 0xDB 	sub.ovf.un 	Subtract native unsigned int from a native unsigned int. Unsigned result shall fit in same size. 	Base instruction
// 0xDC 	endfault 	End fault clause of an exception block. 	Base instruction
// 0xDC 	endfinally 	End finally clause of an exception block. 	Base instruction
// 0xDD 	leave <int32 (target)> 	Exit a protected region of code. 	Base instruction
// 0xDE 	leave.s <int8 (target)> 	Exit a protected region of code, short form. 	Base instruction
// 0xDF 	stind.i 	Store value of type native int into memory at address. 	Base instruction
// 0xE0 	conv.u 	Convert to native unsigned int, pushing native int on stack. 	Base instruction
// 0xFE 0x00 	arglist 	Return argument list handle for the current method. 	Base instruction
// 0xFE 0x01 	ceq 	Push 1 (of type int32) if value1 equals value2, else push 0. 	Base instruction
// 0xFE 0x02 	cgt 	Push 1 (of type int32) if value1 greater than value2, else push 0. 	Base instruction
// 0xFE 0x03 	cgt.un 	Push 1 (of type int32) if value1 greater than value2, unsigned or unordered, else push 0. 	Base instruction
// 0xFE 0x04 	clt 	Push 1 (of type int32) if value1 lower than value2, else push 0. 	Base instruction
// 0xFE 0x05 	clt.un 	Push 1 (of type int32) if value1 lower than value2, unsigned or unordered, else push 0. 	Base instruction
// 0xFE 0x06 	ldftn <method> 	Push a pointer to a method referenced by method, on the stack. 	Base instruction
// 0xFE 0x07 	ldvirtftn <method> 	Push address of virtual method on the stack. 	Object model instruction
// 0xFE 0x09 	ldarg <uint16 (num)> 	Load argument numbered num onto the stack. 	Base instruction
// 0xFE 0x0A 	ldarga <uint16 (argNum)> 	Fetch the address of argument argNum. 	Base instruction
// 0xFE 0x0B 	starg <uint16 (num)> 	Store value to the argument numbered num. 	Base instruction
// 0xFE 0x0C 	ldloc <uint16 (indx)> 	Load local variable of index indx onto stack. 	Base instruction
// 0xFE 0x0D 	ldloca <uint16 (indx)> 	Load address of local variable with index indx. 	Base instruction
// 0xFE 0x0E 	stloc <uint16 (indx)> 	Pop a value from stack into local variable indx. 	Base instruction
// 0xFE 0x0F 	localloc 	Allocate space from the local memory pool. 	Base instruction
// 0xFE 0x11 	endfilter 	End an exception handling filter clause. 	Base instruction
// 0xFE 0x12 	unaligned. (alignment) 	Subsequent pointer instruction might be unaligned. 	Prefix to instruction
// 0xFE 0x13 	volatile. 	Subsequent pointer reference is volatile. 	Prefix to instruction
// 0xFE 0x14 	tail. 	Subsequent call terminates current method. 	Prefix to instruction
// 0xFE 0x15 	initobj <typeTok> 	Initialize the value at address dest. 	Object model instruction
// 0xFE 0x16 	constrained. <thisType> 	Call a virtual method on a type constrained to be type T. 	Prefix to instruction
// 0xFE 0x17 	cpblk 	Copy data from memory to memory. 	Base instruction
// 0xFE 0x18 	initblk 	Set all bytes in a block of memory to a given byte value. 	Base instruction
// 0xFE 0x19

// no. {
//  typecheck,
//  rangecheck,
//  nullcheck
//  }

// 	The specified fault check(s) normally performed as part of the execution of the subsequent instruction can/shall be skipped. 	Prefix to instruction
// 0xFE 0x1A 	rethrow 	Rethrow the current exception. 	Object model instruction
// 0xFE 0x1C 	sizeof <typeTok> 	Push the size, in bytes, of a type as an unsigned int32. 	Object model instruction
// 0xFE 0x1D 	refanytype 	Push the type token stored in a typed reference. 	Object model instruction
// 0xFE 0x1E 	readonly. 	Specify that the subsequent array address operation performs no type check at runtime, and that it returns a controlled-mutability managed pointer. 	Prefix to instruction
