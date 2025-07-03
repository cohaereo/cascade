use binrw::BinReaderExt;
use binrw::binread;
use bitflags::bitflags;
use std::fmt::Display;

use crate::meta::Token;

macro_rules! define_opcodes {
    ($(
        $(#[doc = $description:expr])?
        $index:literal => $name:ident ($($fname:ident : $ftype:ident),*) $asmname:expr
    ),*) => {
        #[derive(Debug, Clone, PartialEq)]
        #[rustfmt::skip]
        #[allow(non_camel_case_types)]
        pub enum RawOpcode {
            $(
                $(#[doc = $description])?
                $name
                    {
                        $(
                            $fname: $ftype,
                        )*
                    },
            )*
            /// Jump to one of n values.
            Switch { targets: Vec<i32> },
        }

        impl RawOpcode {
            pub fn asm_name(&self) -> &'static str {
                match self {
                    $(Self::$name { .. } => $asmname,)*
                    Self::Switch { .. } => "switch",
                }
            }

            pub fn size(&self) -> usize {
                1 + match self {
                    $(Self::$name { .. } => 0 $(+ std::mem::size_of::<$ftype>())*,)*
                    Self::Switch { targets } => 4 + 4 * targets.len(),
                }
            }
        }

        impl Display for RawOpcode {
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

        impl binrw::BinRead for RawOpcode {
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
    /// Do nothing (No operation).
    0x00 => Nop() "nop",
    /// Inform a debugger that a breakpoint has been reached.
    0x01 => Break() "break",
    /// Load argument 0 onto the stack.
    0x02 => LdArg_0() "ldarg.0",
    /// Load argument 1 onto the stack.
    0x03 => LdArg_1() "ldarg.1",
    /// Load argument 2 onto the stack.
    0x04 => LdArg_2() "ldarg.2",
    /// Load argument 3 onto the stack.
    0x05 => LdArg_3() "ldarg.3",

    /// Load local variable 0 onto stack.
    0x06 => LdLoc_0() "ldloc.0",
    /// Load local variable 1 onto stack.
    0x07 => LdLoc_1() "ldloc.1",
    /// Load local variable 2 onto stack.
    0x08 => LdLoc_2() "ldloc.2",
    /// Load local variable 3 onto stack.
    0x09 => LdLoc_3() "ldloc.3",

    /// Pop a value from stack into local variable 0.
    0x0A => StLoc_0() "stloc.0",
    /// Pop a value from stack into local variable 1.
    0x0B => StLoc_1() "stloc.1",
    /// Pop a value from stack into local variable 2.
    0x0C => StLoc_2() "stloc.2",
    /// Pop a value from stack into local variable 3.
    0x0D => StLoc_3() "stloc.3",

    /// Load argument numbered num onto the stack, short form.
    0x0E => LdArg_S(index: u8) "ldarg.s",
    /// Fetch the address of argument argNum, short form.
    0x0F => LdArgA_S(index: u8) "ldarga.s",
    /// Store value to the argument numbered num, short form.
    0x10 => StArg_S(index: u8) "starg.s",
    /// Load local variable of index indx onto stack, short form.
    0x11 => LdLoc_S(index: u8) "ldloc.s",
    /// Load address of local variable with index indx, short form.
    0x12 => LdLocA_S(index: u8) "ldloca.s",
    /// Pop a value from stack into local variable indx, short form.
    0x13 => StLoc_S(index: u8) "stloc.s",

    /// Push a null reference on the stack.
    0x14 => LdNull() "ldnull",
    /// Push -1 onto the stack as int32.
    0x15 => Ldc_I4_M1() "ldc.i4.m1",
    /// Push 0 onto the stack as int32.
    0x16 => Ldc_I4_0() "ldc.i4.0",
    /// Push 1 onto the stack as int32.
    0x17 => Ldc_I4_1() "ldc.i4.1",
    /// Push 2 onto the stack as int32.
    0x18 => Ldc_I4_2() "ldc.i4.2",
    /// Push 3 onto the stack as int32.
    0x19 => Ldc_I4_3() "ldc.i4.3",
    /// Push 4 onto the stack as int32.
    0x1A => Ldc_I4_4() "ldc.i4.4",
    /// Push 5 onto the stack as int32.
    0x1B => Ldc_I4_5() "ldc.i4.5",
    /// Push 6 onto the stack as int32.
    0x1C => Ldc_I4_6() "ldc.i4.6",
    /// Push 7 onto the stack as int32.
    0x1D => Ldc_I4_7() "ldc.i4.7",
    /// Push 8 onto the stack as int32.
    0x1E => Ldc_I4_8() "ldc.i4.8",
    /// Push num onto the stack as int32, short form.
    0x1F => Ldc_I4_S(value: i8) "ldc.i4.s",
    /// Push num of type int32 onto the stack as int32.
    0x20 => Ldc_I4(value: i32) "ldc.i4",
    /// Push num of type int64 onto the stack as int64.
    0x21 => Ldc_I8(value: i64) "ldc.i8",
    /// Push num of type float32 onto the stack as F.
    0x22 => Ldc_R4(value: f32) "ldc.r4",
    /// Push num of type float64 onto the stack as F.
    0x23 => Ldc_R8(value: f64) "ldc.r8",

    /// Duplicate the value on the top of the stack.
    0x25 => Dup() "dup",
    /// Pop value from the stack.
    0x26 => Pop() "pop",

    /// Exit current method and jump to the specified method.
    0x27 => Jmp(method: Token) "jmp",
    /// Call method described by method.
    0x28 => Call(method: Token) "call",
    /// Call method indicated on the stack with arguments described by callsitedescr.
    0x29 => CallInd(callsitedescr: Token) "calli",
    /// Return from method, possibly with a value.
    0x2A => Ret() "ret",
    /// Branch to target, short form.
    0x2B => Br_S(offset: i8) "br.s",
    /// Branch to target if value is zero (false), short form.
    0x2C => Br_False_S(offset: i8) "brfalse.s",
    /// Branch to target if value is non-zero (true), short form.
    0x2D => Br_True_S(offset: i8) "brtrue.s",
    /// Branch to target if equal, short form.
    0x2E => Beq_S(offset: i8) "beq.s",
    /// Branch to target if greater than or equal to, short form.
    0x2F => Bge_S(offset: i8) "bge.s",
    /// Branch to target if greater than, short form.
    0x30 => Bgt_S(offset: i8) "bgt.s",
    /// Branch to target if less than or equal to, short form.
    0x31 => Ble_S(offset: i8) "ble.s",
    /// Branch to target if less than, short form.
    0x32 => Blt_S(offset: i8) "blt.s",
    /// Branch to target if unequal or unordered, short form.
    0x33 => Bne_Un_S(offset: i8) "bne.un.s",
    /// Branch to target if greater than or equal to (unsigned or unordered), short form.
    0x34 => Bge_Un_S(offset: i8) "bge.un.s",
    /// Branch to target if greater than (unsigned or unordered), short form.
    0x35 => Bgt_Un_S(offset: i8) "bgt.un.s",
    /// Branch to target if less than or equal to (unsigned or unordered), short form.
    0x36 => Ble_Un_S(offset: i8) "ble.un.s",
    /// Branch to target if less than (unsigned or unordered), short form.
    0x37 => Blt_Un_S(offset: i8) "blt.un.s",
    /// Branch to target.
    0x38 => Br(offset: i32) "br",
    /// Branch to target if value is zero (false).
    0x39 => Br_False(offset: i32) "brfalse",
    /// Branch to target if value is non-zero (true).
    0x3A => Br_True(offset: i32) "brtrue",
    /// Branch to target if equal.
    0x3B => Beq(offset: i32) "beq",
    /// Branch to target if greater than or equal to.
    0x3C => Bge(offset: i32) "bge",
    /// Branch to target if greater than.
    0x3D => Bgt(offset: i32) "bgt",
    /// Branch to target if less than or equal to.
    0x3E => Ble(offset: i32) "ble",
    /// Branch to target if less than.
    0x3F => Blt(offset: i32) "blt",
    /// Branch to target if unequal or unordered.
    0x40 => Bne_Un(offset: i32) "bne.un",
    /// Branch to target if greater than or equal to (unsigned or unordered).
    0x41 => Bge_Un(offset: i32) "bge.un",
    /// Branch to target if greater than (unsigned or unordered).
    0x42 => Bgt_Un(offset: i32) "bgt.un",
    /// Branch to target if less than or equal to (unsigned or unordered).
    0x43 => Ble_Un(offset: i32) "ble.un",
    /// Branch to target if less than (unsigned or unordered).
    0x44 => Blt_Un(offset: i32) "blt.un",
    // Defined in macro
    // 0x45 => Switch(targets: Vec<i32>) "switch",

    /// Indirect load value of type int8 as int32 on the stack.
    0x46 => LdInd_I1() "ldind.i1",
    /// Indirect load value of type unsigned int8 as int32 on the stack.
    0x47 => LdInd_U1() "ldind.u1",
    /// Indirect load value of type int16 as int32 on the stack.
    0x48 => LdInd_I2() "ldind.i2",
    /// Indirect load value of type unsigned int16 as int32 on the stack.
    0x49 => LdInd_U2() "ldind.u2",
    /// Indirect load value of type int32 as int32 on the stack.
    0x4A => LdInd_I4() "ldind.i4",
    /// Indirect load value of type unsigned int32 as int32 on the stack.
    0x4B => LdInd_U4() "ldind.u4",
    /// Indirect load value of type int64 as int64 on the stack.
    0x4C => LdInd_I8() "ldind.i8",
    /// Indirect load value of type native int as native int on the stack.
    0x4D => LdInd_I() "ldind.i",
    /// Indirect load value of type float32 as F on the stack.
    0x4E => LdInd_R4() "ldind.r4",
    /// Indirect load value of type float64 as F on the stack.
    0x4F => LdInd_R8() "ldind.r8",
    /// Indirect load value of type object ref as O on the stack.
    0x50 => LdInd_Ref() "ldind.ref",
    /// Store value of type object ref (type O) into memory at address.
    0x51 => StInd_Ref() "stind.ref",
    /// Store value of type int8 into memory at address.
    0x52 => StInd_I1() "stind.i1",
    /// Store value of type int16 into memory at address.
    0x53 => StInd_I2() "stind.i2",
    /// Store value of type int32 into memory at address.
    0x54 => StInd_I4() "stind.i4",
    /// Store value of type int64 into memory at address.
    0x55 => StInd_I8() "stind.i8",
    /// Store value of type float32 into memory at address.
    0x56 => StInd_R4() "stind.r4",
    /// Store value of type float64 into memory at address.
    0x57 => StInd_R8() "stind.r8",

    /// Add two values, returning a new value.
    0x58 => Add() "add",
    /// Subtract value2 from value1, returning a new value.
    0x59 => Sub() "sub",
    /// Multiply values.
    0x5A => Mul() "mul",
    /// Divide two values to return a quotient or floating-point result.
    0x5B => Div() "div",
    /// Divide two values, unsigned, returning a quotient.
    0x5C => DivUnsigned() "div.un",
    /// Remainder when dividing one value by another.
    0x5D => Rem() "rem",
    /// Remainder when dividing one unsigned value by another.
    0x5E => RemUnsigned() "rem.un",
    /// Bitwise AND of two integral values, returns an integral value.
    0x5F => And() "and",
    /// Bitwise OR of two integer values, returns an integer.
    0x60 => Or() "or",
    /// Bitwise XOR of integer values, returns an integer.
    0x61 => Xor() "xor",
    /// Shift an integer left (shifting in zeros), return an integer.
    0x62 => Shl() "shl",
    /// Shift an integer right (shift in sign), return an integer.
    0x63 => Shr() "shr",
    /// Shift an integer right (shift in zero), return an integer.
    0x64 => ShrUnsigned() "shr.un",
    /// Negate value.
    0x65 => Neg() "neg",
    /// Bitwise complement.
    0x66 => Not() "not",

    /// Convert to int8, pushing int32 on stack.
    0x67 => Conv_I1() "conv.i1",
    /// Convert to int16, pushing int32 on stack.
    0x68 => Conv_I2() "conv.i2",
    /// Convert to int32, pushing int32 on stack.
    0x69 => Conv_I4() "conv.i4",
    /// Convert to int64, pushing int64 on stack.
    0x6A => Conv_I8() "conv.i8",
    /// Convert to float32, pushing F on stack.
    0x6B => Conv_R4() "conv.r4",
    /// Convert to float64, pushing F on stack.
    0x6C => Conv_R8() "conv.r8",
    /// Convert to unsigned int32, pushing int32 on stack.
    0x6D => Conv_U4() "conv.u4",
    /// Convert to unsigned int64, pushing int64 on stack.
    0x6E => Conv_U8() "conv.u8",

    /// Call a method associated with an object.
    0x6F => CallVirt(method: Token) "callvirt",

    /// Copy a value type from src to dest.
    0x70 => CpObj(typeref: Token) "cpobj",
    /// Copy the value stored at address src to the stack.
    0x71 => LdObj(typeref: Token) "ldobj",
    /// Push a string object for the literal string.
    0x72 => LdStr(string: Token) "ldstr",
    /// Allocate an uninitialized object or value type and call ctor.
    0x73 => NewObj(ctor: Token) "newobj",
    /// Cast obj to class.
    0x74 => CastClass(typeref: Token) "castclass",
    /// Test if obj is an instance of class, returning null or an instance of that class or interface.
    0x75 => IsInst(typeref: Token) "isinst",
    /// Convert unsigned integer to floating-point, pushing F on stack.
    0x76 => Conv_R_Un() "conv.r.un",
    /// Extract a value-type from obj, its boxed representation, and push a controlled-mutability managed pointer to it to the top of the stack.
    0x79 => Unbox(typeref: Token) "unbox",
    /// Throw an exception.
    0x7A => Throw() "throw",
    /// Push the value of field of object (or value type) obj, onto the stack.
    0x7B => LdFld(field: Token) "ldfld",
    /// Push the address of field of object obj on the stack.
    0x7C => LdFlda(field: Token) "ldflda",
    /// Replace the value of field of the object obj with value.
    0x7D => SetFld(field: Token) "stfld",
    /// Push the value of the static field on the stack.
    0x7E => GetFld(field: Token) "ldfld",
    /// Push the address of the static field, field, on the stack.
    0x7F => GetFlda(field: Token) "ldflda",
    /// Replace the value of the static field with val.
    0x80 => StsFld(field: Token) "stsfld",
    /// Store a value of type typeTok at an address.
    0x81 => StObj(typeref: Token) "stobj",
    /// Convert unsigned to an int8 (on the stack as int32) and throw an exception on overflow.
    0x82 => Conv_Ovf_I1_Unsigned() "conv.ovf.i1.un",
    /// Convert unsigned to an int16 (on the stack as int32) and throw an exception on overflow.
    0x83 => Conv_Ovf_I2_Unsigned() "conv.ovf.i2.un",
    /// Convert unsigned to an int32 (on the stack as int32) and throw an exception on overflow.
    0x84 => Conv_Ovf_I4_Unsigned() "conv.ovf.i4.un",
    /// Convert unsigned to an int64 (on the stack as int64) and throw an exception on overflow.
    0x85 => Conv_Ovf_I8_Unsigned() "conv.ovf.i8.un",
    /// Convert unsigned to an unsigned int8 (on the stack as int32) and throw an exception on overflow.
    0x86 => Conv_Ovf_U1_Unsigned() "conv.ovf.u1.un",
    /// Convert unsigned to an unsigned int16 (on the stack as int32) and throw an exception on overflow.
    0x87 => Conv_Ovf_U2_Unsigned() "conv.ovf.u2.un",
    /// Convert unsigned to an unsigned int32 (on the stack as int32) and throw an exception on overflow.
    0x88 => Conv_Ovf_U4_Unsigned() "conv.ovf.u4.un",
    /// Convert unsigned to an unsigned int64 (on the stack as int64) and throw an exception on overflow.
    0x89 => Conv_Ovf_U8_Unsigned() "conv.ovf.u8.un",
    /// Convert unsigned to a native int (on the stack as native int) and throw an exception on overflow.
    0x8A => Conv_Ovf_I_Unsigned() "conv.ovf.i.un",
    /// Convert unsigned to a native unsigned int (on the stack as native int) and throw an exception on overflow.
    0x8B => Conv_Ovf_U_Unsigned() "conv.ovf.u.un",
    /// Convert a boxable value to its boxed form.
    0x8C => Box(typeref: Token) "box",
    /// Create a new array with elements of type etype.
    0x8D => NewArr(typeref: Token) "newarr",
    /// Push the length (of type native unsigned int) of array on the stack.
    0x8E => LdLen() "ldlen",
    /// Load the address of element at index onto the top of the stack.
    0x8F => LdElema(class: Token) "ldelema",
    /// Load the element with type int8 at index onto the top of the stack as an int32.
    0x90 => LdElem_I1() "ldelem.i1",
    /// Load the element with type unsigned int8 at index onto the top of the stack as an int32.
    0x91 => LdElem_U1() "ldelem.u1",
    /// Load the element with type int16 at index onto the top of the stack as an int32.
    0x92 => LdElem_I2() "ldelem.i2",
    /// Load the element with type unsigned int16 at index onto the top of the stack as an int32.
    0x93 => LdElem_U2() "ldelem.u2",
    /// Load the element with type int32 at index onto the top of the stack as an int32.
    0x94 => LdElem_I4() "ldelem.i4",
    /// Load the element with type unsigned int32 at index onto the top of the stack as an int32.
    0x95 => LdElem_U4() "ldelem.u4",
    /// Load the element with type int64 at index onto the top of the stack as an int64.
    0x96 => LdElem_I8() "ldelem.i8",
    /// Load the element with type native int at index onto the top of the stack as a native int.
    0x97 => LdElem_I() "ldelem.i",
    /// Load the element with type float32 at index onto the top of the stack as an F.
    0x98 => LdElem_R4() "ldelem.r4",
    /// Load the element with type float64 at index onto the top of the stack as an F.
    0x99 => LdElem_R8() "ldelem.r8",
    /// Load the element at index onto the top of the stack as an O. The type of the O is the same as the element type of the array pushed on the CIL stack.
    0x9A => LdElem_Ref() "ldelem.ref",
    /// Replace array element at index with the native int value on the stack.
    0x9B => StElem_I() "stelem.i",
    /// Replace array element at index with the int8 value on the stack.
    0x9C => StElem_I1() "stelem.i1",
    /// Replace array element at index with the int16 value on the stack.
    0x9D => StElem_I2() "stelem.i2",
    /// Replace array element at index with the int32 value on the stack.
    0x9E => StElem_I4() "stelem.i4",
    /// Replace array element at index with the int64 value on the stack.
    0x9F => StElem_I8() "stelem.i8",
    /// Replace array element at index with the float32 value on the stack.
    0xA0 => StElem_R4() "stelem.r4",
    /// Replace array element at index with the float64 value on the stack.
    0xA1 => StElem_R8() "stelem.r8",
    /// Replace array element at index with the ref value on the stack.
    0xA2 => StElem_Ref() "stelem.ref",
    /// Load the element at index onto the top of the stack.
    0xA3 => LdElem_Any(typeref: Token) "ldelem.any",
    /// Replace array element at index with the value on the stack.
    0xA4 => StElem_Any(typeref: Token) "stelem.any",
    /// Extract a value-type from obj, its boxed representation, and copy to the top of the stack.
    0xA5 => Unbox_Any(typeref: Token) "unbox.any",
    /// Convert to an int8 (on the stack as int32) and throw an exception on overflow.
    0xB3 => Conv_Ovf_I1() "conv.ovf.i1",
    /// Convert to an unsigned int8 (on the stack as int32) and throw an exception on overflow.
    0xB4 => Conv_Ovf_U1() "conv.ovf.u1",
    /// Convert to an int16 (on the stack as int32) and throw an exception on overflow.
    0xB5 => Conv_Ovf_I2() "conv.ovf.i2",
    /// Convert to an unsigned int16 (on the stack as int32) and throw an exception on overflow.
    0xB6 => Conv_Ovf_U2() "conv.ovf.u2",
    /// Convert to an int32 (on the stack as int32) and throw an exception on overflow.
    0xB7 => Conv_Ovf_I4() "conv.ovf.i4",
    /// Convert to an unsigned int32 (on the stack as int32) and throw an exception on overflow.
    0xB8 => Conv_Ovf_U4() "conv.ovf.u4",
    /// Convert to an int64 (on the stack as int64) and throw an exception on overflow.
    0xB9 => Conv_Ovf_I8() "conv.ovf.i8",
    /// Convert to an unsigned int64 (on the stack as int64) and throw an exception on overflow.
    0xBA => Conv_Ovf_U8() "conv.ovf.u8",
    /// Push the address stored in a typed reference.
    0xC2 => RefAnyVal(class: Token) "refanyval",
    /// Throw ArithmeticException if value is not a finite number.
    0xC3 => CkFinite(typeref: Token) "ckfinite",
    /// Push a typed reference to ptr of type class onto the stack.
    0xC6 => MkRefAny(typeref: Token) "mkrefany",
    /// Convert metadata token to its runtime representation.
    0xD0 => LdToken(token: Token) "ldtoken",
    /// Convert to unsigned int16, pushing int32 on stack.
    0xD1 => Conv_U2() "conv.u2",
    /// Convert to unsigned int8, pushing int32 on stack.
    0xD2 => Conv_U1() "conv.u1",
    /// Convert to native int, pushing native int on stack.
    0xD3 => Conv_I() "conv.i",
    /// Convert to a native int (on the stack as native int) and throw an exception on overflow.
    0xD4 => Conv_Ovf_I() "conv.ovf.i",
    /// Convert to a native unsigned int (on the stack as native int) and throw an exception on overflow.
    0xD5 => Conv_Ovf_U() "conv.ovf.u",
    /// Add signed integer values with overflow check.
    0xD6 => Add_Ovf() "add.ovf",
    /// Add unsigned integer values with overflow check.
    0xD7 => Add_Ovf_Unsigned() "add.ovf.un",
    /// Multiply signed integer values. Signed result shall fit in same size.
    0xD8 => Mul_Ovf() "mul.ovf",
    /// Multiply unsigned integer values. Unsigned result shall fit in same size.
    0xD9 => Mul_Ovf_Unsigned() "mul.ovf.un",
    /// Subtract native int from a native int. Signed result shall fit in same size.
    0xDA => Sub_Ovf() "sub.ovf",
    /// Subtract native unsigned int from a native unsigned int. Unsigned result shall fit in same size.
    0xDB => Sub_Ovf_Unsigned() "sub.ovf.un",
    /// End fault clause of an exception block.
    0xDC => EndFaultOrFinally() "endfault/endfinally",
    /// Exit a protected region of code.
    0xDD => Leave(offset: i32) "leave",
    /// Exit a protected region of code, short form.
    0xDE => Leave_S(offset: i8) "leave.s",
    /// Store value of type native int into memory at address.
    0xDF => StInd_I() "stind.i",
    /// Convert to native unsigned int, pushing native int on stack.
    0xE0 => Conv_U() "conv.u",

    /// Return argument list handle for the current method.
    0xFE_00 => ArgList() "arglist",
    /// Push 1 (of type int32) if value1 equals value2, else push 0.
    0xFE_01 => Ceq() "ceq",
    /// Push 1 (of type int32) if value1 greater than value2, else push 0.
    0xFE_02 => Cgt() "cgt",
    /// Push 1 (of type int32) if value1 greater than value2, unsigned or unordered, else push 0.
    0xFE_03 => Cgt_Un() "cgt.un",
    /// Push 1 (of type int32) if value1 lower than value2, else push 0.
    0xFE_04 => Clt() "clt",
    /// Push 1 (of type int32) if value1 lower than value2, unsigned or unordered, else push 0.
    0xFE_05 => Clt_Un() "clt.un",
    /// Push a pointer to a method referenced by method, on the stack.
    0xFE_06 => LdFtn(method: Token) "ldftn",
    /// Push address of virtual method on the stack.
    0xFE_07 => LdVirtFtn(method: Token) "ldvirtftn",
    /// Load argument numbered num onto the stack.
    0xFE_09 => LdArg(index: u16) "ldarg",
    /// Fetch the address of argument argNum.
    0xFE_0A => LdArgA(index: u16) "ldarga",
    /// Store value to the argument numbered num.
    0xFE_0B => StArg(index: u16) "starg",
    /// Load local variable of index indx onto stack.
    0xFE_0C => LdLoc(index: u16) "ldloc",
    /// Load address of local variable with index indx.
    0xFE_0D => LdLocA(index: u16) "ldloca",
    /// Pop a value from stack into local variable indx.
    0xFE_0E => StLoc(index: u16) "stloc",
    /// Allocate space from the local memory pool.
    0xFE_0F => LocAlloc(size: u32) "localloc",
    /// End an exception handling filter clause.
    0xFE_11 => EndFilter() "endfilter",
    /// Subsequent pointer instruction might be unaligned.
    0xFE_12 => Unaligned() "unaligned.",
    /// Subsequent pointer reference is volatile.
    0xFE_13 => Volatile() "volatile.",
    /// Subsequent call terminates current method.
    0xFE_14 => Tail() "tail.",
    /// Initialize the value at address dest.
    0xFE_15 => InitObj(type_token: Token) "initobj",
    /// Call a virtual method on a type constrained to be type T.
    0xFE_16 => Constrained(this_type: Token) "constrained.",
    /// Copy data from memory to memory.
    0xFE_17 => CpBlk() "cpblk",
    /// Set all bytes in a block of memory to a given byte value.
    0xFE_18 => InitBlk() "initblk",
    /// Push the size, in bytes, of a type as an unsigned int32.
    0xFE_1C => SizeOf(type_token: Token) "sizeof"
}

impl RawOpcode {
    pub fn is_branch(&self) -> bool {
        matches!(
            self,
            Self::Br { .. }
                | Self::Br_False { .. }
                | Self::Br_True { .. }
                | Self::Beq { .. }
                | Self::Bge { .. }
                | Self::Bgt { .. }
                | Self::Ble { .. }
                | Self::Blt { .. }
                | Self::Bne_Un { .. }
                | Self::Bge_Un { .. }
                | Self::Bgt_Un { .. }
                | Self::Ble_Un { .. }
                | Self::Blt_Un { .. }
                | Self::Br_S { .. }
                | Self::Br_False_S { .. }
                | Self::Br_True_S { .. }
                | Self::Beq_S { .. }
                | Self::Bge_S { .. }
                | Self::Bgt_S { .. }
                | Self::Ble_S { .. }
                | Self::Blt_S { .. }
                | Self::Bne_Un_S { .. }
                | Self::Bge_Un_S { .. }
                | Self::Bgt_Un_S { .. }
                | Self::Ble_Un_S { .. }
                | Self::Blt_Un_S { .. }
        )
    }

    /// If this opcode is a branch, returns the offset relative to the start of THIS instruction.
    pub fn branch_offset(&self) -> Option<i32> {
        Some(
            match self {
                Self::Br { offset } => *offset,
                Self::Br_False { offset } => *offset,
                Self::Br_True { offset } => *offset,
                Self::Beq { offset } => *offset,
                Self::Bge { offset } => *offset,
                Self::Bgt { offset } => *offset,
                Self::Ble { offset } => *offset,
                Self::Blt { offset } => *offset,
                Self::Bne_Un { offset } => *offset,
                Self::Bge_Un { offset } => *offset,
                Self::Bgt_Un { offset } => *offset,
                Self::Ble_Un { offset } => *offset,
                Self::Blt_Un { offset } => *offset,
                Self::Br_S { offset } => i32::from(*offset),
                Self::Br_False_S { offset } => i32::from(*offset),
                Self::Br_True_S { offset } => i32::from(*offset),
                Self::Beq_S { offset } => i32::from(*offset),
                Self::Bge_S { offset } => i32::from(*offset),
                Self::Bgt_S { offset } => i32::from(*offset),
                Self::Ble_S { offset } => i32::from(*offset),
                Self::Blt_S { offset } => i32::from(*offset),
                Self::Bne_Un_S { offset } => i32::from(*offset),
                Self::Bge_Un_S { offset } => i32::from(*offset),
                Self::Bgt_Un_S { offset } => i32::from(*offset),
                Self::Ble_Un_S { offset } => i32::from(*offset),
                Self::Blt_Un_S { offset } => i32::from(*offset),
                _ => return None, // Not a branch
            } + self.size() as i32,
        )
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DisabledFaultChecks: u8 {
        const TYPE_CHECK = 0x01;
        const RANGE_CHECK = 0x02;
        const NULL_CHECK = 0x04;
    }
}
