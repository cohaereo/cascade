use std::{fmt::Display, io::Cursor};

use binrw::{BinRead, BinReaderExt, binread};
use int_enum::IntEnum;

use crate::{
    Result, bitfield, image::CilImage, meta::Token, signature, tables::TypeDefOrRef,
    util::PackedU32,
};

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SignatureKind {
    StandaloneMethod,
    Field,
    LocalVar,
    Property,
}

impl TryFrom<u8> for SignatureKind {
    type Error = &'static str;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value & 0xF {
            0x0..=0x5 => Ok(SignatureKind::StandaloneMethod),
            0x6 => Ok(SignatureKind::Field),
            0x7 => Ok(SignatureKind::LocalVar),
            0x8 => Ok(SignatureKind::Property),
            _ => Err("Invalid signature kind"),
        }
    }
}

impl BinRead for SignatureKind {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let value = reader.read_le::<u8>()?;
        Self::try_from(value).map_err(|err| binrw::Error::NoVariantMatch {
            pos: reader.stream_position().unwrap(),
        })
    }
}

#[binread]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Element {
    #[br(magic(0x00u8))] End,
    #[br(magic(0x01u8))] Void,
    #[br(magic(0x02u8))] Boolean,
    #[br(magic(0x03u8))] Char,
    #[br(magic(0x04u8))] I1,
    #[br(magic(0x05u8))] U1,
    #[br(magic(0x06u8))] I2,
    #[br(magic(0x07u8))] U2,
    #[br(magic(0x08u8))] I4,
    #[br(magic(0x09u8))] U4,
    #[br(magic(0x0Au8))] I8,
    #[br(magic(0x0Bu8))] U8,
    #[br(magic(0x0Cu8))] R4,
    #[br(magic(0x0Du8))] R8,
    #[br(magic(0x0Eu8))] String,
    #[br(magic(0x0Fu8))] Ptr(Box<Self>),
    #[br(magic(0x10u8))] ByRef(Box<Self>),
    #[br(magic(0x11u8))] ValueType(TypeDefOrRef),
    #[br(magic(0x12u8))] Class(TypeDefOrRef),
    #[br(magic(0x13u8))] Var(PackedU32),
    #[br(magic(0x15u8))] GenericInst {
        generic_type: Box<Self>,
        #[br(temp)]
        generic_arg_count: u8,
        #[br(count = generic_arg_count)]
        generic_args: Vec<Self>,
    },
    #[br(magic(0x18u8))] IntPtr,
    #[br(magic(0x19u8))] UIntPtr,
    #[br(magic(0x1Bu8))] FnPtr(Box<StandaloneMethodSignature>),
    #[br(magic(0x1Cu8))] Object,
    #[br(magic(0x1Du8))] SzArray(Box<Self>),
    #[br(magic(0x1Eu8))] MVar(PackedU32),
    #[br(magic(0x1Fu8))] CModRequired(TypeDefOrRef),
    #[br(magic(0x20u8))] CModOptional(TypeDefOrRef),
    #[br(magic(0x45u8))] Pinned(Box<Self>),
}

impl Element {
    pub fn debug_print(&self, image: &CilImage) -> String {
        let s = match self {
            Element::End => "<end>",
            Element::Void => "void",
            Element::Boolean => "bool",
            Element::Char => "char",
            Element::I1 => "int8",
            Element::U1 => "uint8",
            Element::I2 => "int16",
            Element::U2 => "uint16",
            Element::I4 => "int32",
            Element::U4 => "uint32",
            Element::I8 => "int64",
            Element::U8 => "uint64",
            Element::R4 => "float32",
            Element::R8 => "float64",
            Element::Ptr(inner) => &format!("*{}", inner.debug_print(image)),
            Element::ByRef(inner) => &format!("ref {}", inner.debug_print(image)),
            Element::ValueType(token) => {
                if let Some(typename) = token.name_with_namespace(image) {
                    &format!("{typename}")
                } else {
                    &format!("<unk:{:?}>", token)
                }
            }
            Element::Class(token) => {
                if let Some(typename) = token.name_with_namespace(image) {
                    &format!("{typename}")
                } else {
                    &format!("<unk:{:?}>", token)
                }
            }
            Element::Var(index) => &format!("var{}", index.0),
            Element::GenericInst {
                generic_type,
                generic_args,
                ..
            } => {
                let args = generic_args
                    .iter()
                    .map(|arg| arg.debug_print(image))
                    .collect::<Vec<_>>()
                    .join(", ");
                &format!("{}<{}>", generic_type.debug_print(image), args)
            }
            Element::String => "string",
            Element::IntPtr => "nint",
            Element::UIntPtr => "nuint",
            Element::FnPtr(signature) => &format!("{}", signature.debug_print(image)),
            Element::Object => "object",
            Element::SzArray(inner) => &format!("{}[]", inner.debug_print(image)),
            Element::MVar(index) => &format!("mvar{}", index.0),
            Element::CModRequired(type_def_or_ref) => {
                &format!("cmodreq({:?})", type_def_or_ref.name_with_namespace(image))
            }
            Element::CModOptional(type_def_or_ref) => {
                &format!("cmodopt({:?})", type_def_or_ref.name_with_namespace(image))
            }
            Element::Pinned(boxed_element) => {
                &format!("pinned {}", boxed_element.debug_print(image))
            }
        };

        s.to_string()
    }
}

bitfield! {
    pub struct StandaloneMethodSigHeader : u8 {
        enum call_type: MethodCallType @ 0x03 >> 0,
        flag has_this: bool @ 0x20,
        flag explicit_this: bool @ 0x40
    }
}

impl Default for StandaloneMethodSigHeader {
    fn default() -> Self {
        Self(0)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntEnum)]
pub enum MethodCallType {
    Default = 0,
    C = 1,
    StdCall = 2,
    ThisCall = 3,
    FastCall = 4,
    Vararg = 5,
}

#[binread]
#[derive(Debug, Clone, PartialEq)]
pub struct StandaloneMethodSignature {
    pub header: StandaloneMethodSigHeader,
    #[br(temp)]
    count: PackedU32,
    pub return_type: Element,
    #[br(count = count.0 as usize)]
    pub parameters: Vec<Element>,
}

impl Default for StandaloneMethodSignature {
    fn default() -> Self {
        Self {
            header: StandaloneMethodSigHeader::default(),
            return_type: Element::Void,
            parameters: Vec::new(),
        }
    }
}

impl StandaloneMethodSignature {
    pub fn parse(blob: &[u8]) -> Result<Self> {
        let mut reader = Cursor::new(blob);
        Ok(reader.read_le()?)
    }

    pub fn debug_print(&self, image: &CilImage) -> String {
        let mut s = String::new();

        if self.header.has_this() {
            s.push_str("this ");
        }

        s.push_str("fn(");

        if !self.parameters.is_empty() {
            s.push_str(
                &self
                    .parameters
                    .iter()
                    .map(|p| p.debug_print(image))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }

        s.push_str(") -> ");
        s.push_str(&self.return_type.debug_print(image));

        s.to_string()
    }
}
