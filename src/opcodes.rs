//! This module provides an abstracted representation of opcodes for easier consumption
//!
//! For example, opcode variants such as `ldarg.0`, `ldarg.1`, etc are represented as a single `LdArg(0)` and `LdArg(1)` respectively instead of having separate variants for each argument index.

use cil::{meta::Token, opcodes::RawOpcode, signature::Element};

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Misc
    Nop,
    Break,

    // Math
    Add(OverflowCheck),
    Subtract(OverflowCheck),
    Multiply(OverflowCheck),
    Divide {
        unsigned: bool,
    },
    Remainder {
        unsigned: bool,
    },
    Compare {
        comparison: Comparison,
        unsigned: bool,
    },

    // Bitwise
    ShiftLeft,
    ShiftRight,
    And,
    Or,
    Xor,

    // Constants
    /// Load a user string onto the stack
    LoadString(Token),
    LoadConstantI4(i32),
    LoadConstantI8(i64),
    LoadConstantR4(f32),
    LoadConstantR8(f64),

    // Load/Store
    LoadArg(u16),
    LoadArgAddress(u16),
    LoadLocal(u16),
    LoadLocalAddress(u16),
    StoreLocal(u16),

    // Flow control
    Call(Token),
    Return,

    /// Unconditional branch to a label
    Branch(i32),
    BranchConditional {
        offset: i32,
        comparison: Comparison,
        unsigned: bool,
    },
    Switch {
        targets: Vec<i32>,
    },

    // Object manipulation
    SetField {
        field: Token,
    },

    // Conversions
    ConvertToI1,
    ConvertToI2,
    ConvertToI4,
    ConvertToI8,
}

impl From<RawOpcode> for Opcode {
    fn from(raw: RawOpcode) -> Self {
        match raw {
            RawOpcode::Add {} => Opcode::Add(OverflowCheck::Off),
            RawOpcode::Add_Ovf {} => Opcode::Add(OverflowCheck::Signed),
            RawOpcode::Add_Ovf_Unsigned {} => Opcode::Add(OverflowCheck::Unsigned),
            RawOpcode::And {} => Self::And,
            // RawOpcode::ArgList {} => todo!(),
            RawOpcode::Beq { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::Equal,
                unsigned: false,
            },
            RawOpcode::Beq_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::Equal,
                unsigned: false,
            },
            RawOpcode::Bge { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::GreaterOrEqual,
                unsigned: false,
            },
            RawOpcode::Bge_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::GreaterOrEqual,
                unsigned: false,
            },
            RawOpcode::Bge_Un { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::GreaterOrEqual,
                unsigned: true,
            },
            RawOpcode::Bge_Un_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::GreaterOrEqual,
                unsigned: true,
            },
            RawOpcode::Bgt { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::Greater,
                unsigned: false,
            },
            RawOpcode::Bgt_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::Greater,
                unsigned: false,
            },
            RawOpcode::Bgt_Un { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::Greater,
                unsigned: true,
            },
            RawOpcode::Bgt_Un_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::Greater,
                unsigned: true,
            },
            RawOpcode::Ble { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::LessOrEqual,
                unsigned: false,
            },
            RawOpcode::Ble_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::LessOrEqual,
                unsigned: false,
            },
            RawOpcode::Ble_Un { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::LessOrEqual,
                unsigned: true,
            },
            RawOpcode::Ble_Un_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::LessOrEqual,
                unsigned: true,
            },
            RawOpcode::Blt { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::Less,
                unsigned: false,
            },
            RawOpcode::Blt_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::Less,
                unsigned: false,
            },
            RawOpcode::Blt_Un { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::Less,
                unsigned: true,
            },
            RawOpcode::Blt_Un_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::Less,
                unsigned: true,
            },
            RawOpcode::Bne_Un { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::NotEqual,
                unsigned: true,
            },
            RawOpcode::Bne_Un_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::NotEqual,
                unsigned: true,
            },
            RawOpcode::Br_False { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::Zero,
                unsigned: false,
            },
            RawOpcode::Br_False_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::Zero,
                unsigned: false,
            },
            RawOpcode::Br_True { offset } => Self::BranchConditional {
                offset,
                comparison: Comparison::One,
                unsigned: false,
            },
            RawOpcode::Br_True_S { offset } => Self::BranchConditional {
                offset: offset as i32,
                comparison: Comparison::One,
                unsigned: false,
            },
            RawOpcode::Br { offset } => Self::Branch(offset),
            RawOpcode::Br_S { offset } => Self::Branch(offset as i32),

            // RawOpcode::Box { typeref } => todo!(),
            // RawOpcode::Break {} => todo!(),
            RawOpcode::Call { method } => Self::Call(method),
            // RawOpcode::CallInd { callsitedescr } => todo!(),
            // RawOpcode::CallVirt { method } => todo!(),
            // RawOpcode::CastClass { typeref } => todo!(),
            RawOpcode::Ceq {} => Self::Compare {
                comparison: Comparison::Equal,
                unsigned: false,
            },
            RawOpcode::Cgt {} => Self::Compare {
                comparison: Comparison::Greater,
                unsigned: false,
            },
            RawOpcode::Cgt_Un {} => Self::Compare {
                comparison: Comparison::Greater,
                unsigned: true,
            },
            // RawOpcode::CkFinite { typeref } => todo!(),
            RawOpcode::Clt {} => Self::Compare {
                comparison: Comparison::Less,
                unsigned: false,
            },
            RawOpcode::Clt_Un {} => Self::Compare {
                comparison: Comparison::Less,
                unsigned: true,
            },
            // RawOpcode::Constrained { this_type } => todo!(),
            // RawOpcode::Conv_I {} => todo!(),
            RawOpcode::Conv_I1 {} => Self::ConvertToI1,
            RawOpcode::Conv_I2 {} => Self::ConvertToI2,
            RawOpcode::Conv_I4 {} => Self::ConvertToI4,
            RawOpcode::Conv_I8 {} => Self::ConvertToI8,
            // RawOpcode::Conv_Ovf_I {} => todo!(),
            // RawOpcode::Conv_Ovf_I1 {} => todo!(),
            // RawOpcode::Conv_Ovf_I1_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_I2 {} => todo!(),
            // RawOpcode::Conv_Ovf_I2_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_I4 {} => todo!(),
            // RawOpcode::Conv_Ovf_I4_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_I8 {} => todo!(),
            // RawOpcode::Conv_Ovf_I8_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_I_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_U {} => todo!(),
            // RawOpcode::Conv_Ovf_U1 {} => todo!(),
            // RawOpcode::Conv_Ovf_U1_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_U2 {} => todo!(),
            // RawOpcode::Conv_Ovf_U2_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_U4 {} => todo!(),
            // RawOpcode::Conv_Ovf_U4_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_U8 {} => todo!(),
            // RawOpcode::Conv_Ovf_U8_Unsigned {} => todo!(),
            // RawOpcode::Conv_Ovf_U_Unsigned {} => todo!(),
            // RawOpcode::Conv_R4 {} => todo!(),
            // RawOpcode::Conv_R8 {} => todo!(),
            // RawOpcode::Conv_R_Un {} => todo!(),
            // RawOpcode::Conv_U {} => todo!(),
            // RawOpcode::Conv_U1 {} => todo!(),
            // RawOpcode::Conv_U2 {} => todo!(),
            // RawOpcode::Conv_U4 {} => todo!(),
            // RawOpcode::Conv_U8 {} => todo!(),
            // RawOpcode::CpBlk {} => todo!(),
            // RawOpcode::CpObj { typeref } => todo!(),
            RawOpcode::Div {} => Self::Divide { unsigned: false },
            RawOpcode::DivUnsigned {} => Self::Divide { unsigned: true },
            // RawOpcode::Dup {} => todo!(),
            // RawOpcode::EndFaultOrFinally {} => todo!(),
            // RawOpcode::EndFilter {} => todo!(),
            // RawOpcode::GetFld { field } => todo!(),
            // RawOpcode::GetFlda { field } => todo!(),
            // RawOpcode::InitBlk {} => todo!(),
            // RawOpcode::InitObj { type_token } => todo!(),
            // RawOpcode::IsInst { typeref } => todo!(),
            // RawOpcode::Jmp { method } => todo!(),
            RawOpcode::LdArg { index } => Opcode::LoadArg(index),
            RawOpcode::LdArgA { index } => Opcode::LoadArgAddress(index),
            RawOpcode::LdArgA_S { index } => Opcode::LoadArgAddress(index as u16),
            RawOpcode::LdArg_0 {} => Opcode::LoadArg(0),
            RawOpcode::LdArg_1 {} => Opcode::LoadArg(1),
            RawOpcode::LdArg_2 {} => Opcode::LoadArg(2),
            RawOpcode::LdArg_3 {} => Opcode::LoadArg(3),
            RawOpcode::LdArg_S { index } => Opcode::LoadArg(index as u16),
            // RawOpcode::LdElem_Any { typeref } => todo!(),
            // RawOpcode::LdElem_I {} => todo!(),
            // RawOpcode::LdElem_I1 {} => todo!(),
            // RawOpcode::LdElem_I2 {} => todo!(),
            // RawOpcode::LdElem_I4 {} => todo!(),
            // RawOpcode::LdElem_I8 {} => todo!(),
            // RawOpcode::LdElem_R4 {} => todo!(),
            // RawOpcode::LdElem_R8 {} => todo!(),
            // RawOpcode::LdElem_Ref {} => todo!(),
            // RawOpcode::LdElem_U1 {} => todo!(),
            // RawOpcode::LdElem_U2 {} => todo!(),
            // RawOpcode::LdElem_U4 {} => todo!(),
            // RawOpcode::LdElema { class } => todo!(),
            // RawOpcode::LdFld { field } => todo!(),
            // RawOpcode::LdFlda { field } => todo!(),
            // RawOpcode::LdFtn { method } => todo!(),
            // RawOpcode::LdInd_I {} => todo!(),
            // RawOpcode::LdInd_I1 {} => todo!(),
            // RawOpcode::LdInd_I2 {} => todo!(),
            // RawOpcode::LdInd_I4 {} => todo!(),
            // RawOpcode::LdInd_I8 {} => todo!(),
            // RawOpcode::LdInd_R4 {} => todo!(),
            // RawOpcode::LdInd_R8 {} => todo!(),
            // RawOpcode::LdInd_Ref {} => todo!(),
            // RawOpcode::LdInd_U1 {} => todo!(),
            // RawOpcode::LdInd_U2 {} => todo!(),
            // RawOpcode::LdInd_U4 {} => todo!(),
            // RawOpcode::LdLen {} => todo!(),
            RawOpcode::LdLoc { index } => Self::LoadLocal(index),
            RawOpcode::LdLocA { index } => Self::LoadLocalAddress(index),
            RawOpcode::LdLocA_S { index } => Self::LoadLocalAddress(index as u16),
            RawOpcode::LdLoc_0 {} => Self::LoadLocal(0),
            RawOpcode::LdLoc_1 {} => Self::LoadLocal(1),
            RawOpcode::LdLoc_2 {} => Self::LoadLocal(2),
            RawOpcode::LdLoc_3 {} => Self::LoadLocal(3),
            RawOpcode::LdLoc_S { index } => Self::LoadLocal(index as u16),
            // RawOpcode::LdNull {} => todo!(),
            // RawOpcode::LdObj { typeref } => todo!(),
            RawOpcode::LdStr { string } => Self::LoadString(string),
            // RawOpcode::LdToken { token } => todo!(),
            // RawOpcode::LdVirtFtn { method } => todo!(),
            RawOpcode::Ldc_I4 { value } => Self::LoadConstantI4(value),
            RawOpcode::Ldc_I4_0 {} => Self::LoadConstantI4(0),
            RawOpcode::Ldc_I4_1 {} => Self::LoadConstantI4(1),
            RawOpcode::Ldc_I4_2 {} => Self::LoadConstantI4(2),
            RawOpcode::Ldc_I4_3 {} => Self::LoadConstantI4(3),
            RawOpcode::Ldc_I4_4 {} => Self::LoadConstantI4(4),
            RawOpcode::Ldc_I4_5 {} => Self::LoadConstantI4(5),
            RawOpcode::Ldc_I4_6 {} => Self::LoadConstantI4(6),
            RawOpcode::Ldc_I4_7 {} => Self::LoadConstantI4(7),
            RawOpcode::Ldc_I4_8 {} => Self::LoadConstantI4(8),
            RawOpcode::Ldc_I4_M1 {} => Self::LoadConstantI4(-1),
            RawOpcode::Ldc_I4_S { value } => Self::LoadConstantI4(value as i32),
            RawOpcode::Ldc_I8 { value } => Self::LoadConstantI8(value),
            RawOpcode::Ldc_R4 { value } => Self::LoadConstantR4(value),
            RawOpcode::Ldc_R8 { value } => Self::LoadConstantR8(value),
            // RawOpcode::Leave { offset } => todo!(),
            // RawOpcode::Leave_S { offset } => todo!(),
            // RawOpcode::LocAlloc { size } => todo!(),
            // RawOpcode::MkRefAny { typeref } => todo!(),
            RawOpcode::Mul {} => Self::Multiply(OverflowCheck::Off),
            RawOpcode::Mul_Ovf {} => Self::Multiply(OverflowCheck::Signed),
            RawOpcode::Mul_Ovf_Unsigned {} => Self::Multiply(OverflowCheck::Unsigned),
            // RawOpcode::Neg {} => todo!(),
            // RawOpcode::NewArr { typeref } => todo!(),
            // RawOpcode::NewObj { ctor } => todo!(),
            RawOpcode::Nop {} => Self::Nop,
            // RawOpcode::Not {} => todo!(),
            RawOpcode::Or {} => Self::Or,
            // RawOpcode::Pop {} => todo!(),
            // RawOpcode::RefAnyVal { class } => todo!(),
            RawOpcode::Rem {} => Self::Remainder { unsigned: false },
            RawOpcode::RemUnsigned {} => Self::Remainder { unsigned: true },
            RawOpcode::Ret {} => Self::Return,
            RawOpcode::SetFld { field } => Self::SetField { field },
            RawOpcode::Shl {} => Self::ShiftLeft,
            RawOpcode::Shr {} => Self::ShiftRight,
            // RawOpcode::ShrUnsigned {} => todo!(),
            // RawOpcode::SizeOf { type_token } => todo!(),
            // RawOpcode::StArg { index } => todo!(),
            // RawOpcode::StArg_S { index } => todo!(),
            // RawOpcode::StElem_Any { typeref } => todo!(),
            // RawOpcode::StElem_I {} => todo!(),
            // RawOpcode::StElem_I1 {} => todo!(),
            // RawOpcode::StElem_I2 {} => todo!(),
            // RawOpcode::StElem_I4 {} => todo!(),
            // RawOpcode::StElem_I8 {} => todo!(),
            // RawOpcode::StElem_R4 {} => todo!(),
            // RawOpcode::StElem_R8 {} => todo!(),
            // RawOpcode::StElem_Ref {} => todo!(),
            // RawOpcode::StInd_I {} => todo!(),
            // RawOpcode::StInd_I1 {} => todo!(),
            // RawOpcode::StInd_I2 {} => todo!(),
            // RawOpcode::StInd_I4 {} => todo!(),
            // RawOpcode::StInd_I8 {} => todo!(),
            // RawOpcode::StInd_R4 {} => todo!(),
            // RawOpcode::StInd_R8 {} => todo!(),
            // RawOpcode::StInd_Ref {} => todo!(),
            RawOpcode::StLoc { index } => Self::StoreLocal(index),
            RawOpcode::StLoc_0 {} => Self::StoreLocal(0),
            RawOpcode::StLoc_1 {} => Self::StoreLocal(1),
            RawOpcode::StLoc_2 {} => Self::StoreLocal(2),
            RawOpcode::StLoc_3 {} => Self::StoreLocal(3),
            RawOpcode::StLoc_S { index } => Self::StoreLocal(index as u16),
            // RawOpcode::StObj { typeref } => todo!(),
            // RawOpcode::StsFld { field } => todo!(),
            RawOpcode::Sub {} => Self::Subtract(OverflowCheck::Off),
            RawOpcode::Sub_Ovf {} => Self::Subtract(OverflowCheck::Signed),
            RawOpcode::Sub_Ovf_Unsigned {} => Self::Subtract(OverflowCheck::Unsigned),
            RawOpcode::Switch { targets } => Self::Switch { targets },
            // RawOpcode::Tail {} => todo!(),
            // RawOpcode::Throw {} => todo!(),
            // RawOpcode::Unaligned {} => todo!(),
            // RawOpcode::Unbox { typeref } => todo!(),
            // RawOpcode::Unbox_Any { typeref } => todo!(),
            // RawOpcode::Volatile {} => todo!(),
            RawOpcode::Xor {} => Self::Xor,
            u => unimplemented!("Opcode conversion not implemented for raw opcode {:?}", u),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverflowCheck {
    Off,
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparison {
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    NotEqual,

    One,
    Zero,
}

impl Comparison {
    /// Returns true if the comparison is either `One` or `Zero`.
    pub fn is_true_false(&self) -> bool {
        matches!(self, Comparison::One | Comparison::Zero)
    }

    /// Returns the operator string for the comparison.
    ///
    /// When the comparison is not `One` or `Zero`, the caller should append the right-hand value for comparison to the operator string.
    pub fn operator<P: AsRef<str>>(&self, lhs: P, rhs: Option<P>) -> String {
        let operator = match self {
            Comparison::Equal => "==",
            Comparison::Greater => ">",
            Comparison::GreaterOrEqual => ">=",
            Comparison::Less => "<",
            Comparison::LessOrEqual => "<=",
            Comparison::NotEqual => "!=",
            Comparison::One => "== true",
            Comparison::Zero => "== false",
        };

        if rhs.is_some() {
            assert!(
                !matches!(self, Comparison::One | Comparison::Zero),
                "Cannot compare true/false with a value"
            );
        }

        if let Some(rhs) = rhs {
            format!("{} {} {}", lhs.as_ref(), operator, rhs.as_ref())
        } else {
            format!("{} {}", lhs.as_ref(), operator)
        }
    }
}
