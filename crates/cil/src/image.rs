use binrw::BinReaderExt;
use std::{
    fmt::Display,
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
};

use object::{Object, ObjectSection, pe::IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR, read::pe::PeFile32};

use crate::{
    Result,
    error::Error,
    meta::{Guid, PhysicalMetadata, Token, TokenKind},
    opcodes::RawOpcode,
    signature::StandaloneMethodSignature,
    tables::{self, MemberRefParent, TypeDefOrRef, member},
};
use crate::{
    header::CliHeader,
    strings::{BlobHeap, StringHeap, UserStringHeap},
};

pub struct TypeName {
    pub namespace: String,
    pub name: String,
}

impl TypeName {
    pub fn path_cxx(&self) -> String {
        self.to_string().replace(".", "::")
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.namespace.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}.{}", self.namespace, self.name)
        }
    }
}

pub struct CilImage {
    pub code_base: u32,
    pub header: CliHeader,

    pub guids: Vec<Guid>,
    pub strings: StringHeap,
    pub user_strings: UserStringHeap,
    pub blobs: BlobHeap,

    // Tables
    pub modules: Vec<tables::Module>,
    pub type_refs: Vec<tables::TypeRef>,
    pub type_defs: Vec<tables::TypeDef>,
    pub fields: Vec<tables::Field>,
    pub method_defs: Vec<(tables::Method, MethodHeader, Vec<(u32, RawOpcode)>)>,
    pub params: Vec<tables::Param>,
    pub member_refs: Vec<tables::MemberRef>,
    pub custom_attributes: Vec<tables::CustomAttribute>,
    pub stand_alone_sigs: Vec<tables::StandAloneSig>,
    pub assemblies: Vec<tables::Assembly>,
    pub assembly_refs: Vec<tables::AssemblyRef>,
    pub interface_impls: Vec<tables::InterfaceImpl>,
    pub constants: Vec<tables::Constant>,
    pub decl_security: Vec<tables::DeclSecurity>,
    pub class_layouts: Vec<tables::ClassLayout>,
    pub field_layouts: Vec<tables::FieldLayout>,
    pub property_maps: Vec<tables::PropertyMap>,
    pub properties: Vec<tables::Property>,
    pub method_semantics: Vec<tables::MethodSemantics>,
    pub method_impls: Vec<tables::MethodImpl>,
    pub type_specs: Vec<tables::TypeSpec>,
    pub impl_maps: Vec<tables::ImplMap>,
    pub nested_classes: Vec<tables::NestedClass>,
    pub generic_params: Vec<tables::GenericParam>,
    pub method_specs: Vec<tables::MethodSpec>,
    pub generic_param_constraints: Vec<tables::GenericParamConstraint>,
    pub field_rvas: Vec<tables::FieldRva>,
}

impl CilImage {
    pub fn load<A: AsRef<Path>>(path: A) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::read(&data)
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let obj = PeFile32::parse(data).map_err(|_| Error::InvalidCilImage)?;
        let dir = obj
            .data_directories()
            .get(IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR)
            .ok_or(Error::InvalidCilImage)?;

        let code_base = obj
            .nt_headers()
            .optional_header
            .base_of_code
            .get(object::LittleEndian);

        let cli_header_offset = dir.virtual_address.get(object::LittleEndian) - code_base;
        let code_section = obj
            .section_by_name(".text")
            .expect("No .text section found");
        let mut c = Cursor::new(code_section.data().unwrap());
        c.set_position(cli_header_offset as u64);
        let cli_header: CliHeader = c.read_le()?;

        let metadata_offset = cli_header.physical_metadata.rva as u64 - code_base as u64;
        c.set_position(metadata_offset);
        let physical_metadata: PhysicalMetadata = c.read_le()?;

        let strings = {
            let string_stream = physical_metadata
                .streams
                .iter()
                .find(|s| s.name == "#Strings")
                .expect("No #Strings stream found");

            let mut string_data = vec![0u8; string_stream.size as usize];
            c.set_position(string_stream.offset as u64 + metadata_offset);
            c.read_exact(&mut string_data)?;
            StringHeap::new(string_data)
        };

        let guids = {
            let guid_stream = physical_metadata
                .streams
                .iter()
                .find(|s| s.name == "#GUID")
                .expect("No #GUID stream found");
            let mut guid_data = vec![0u8; guid_stream.size as usize];
            c.seek(SeekFrom::Start(guid_stream.offset as u64 + metadata_offset))
                .unwrap();
            c.read_exact(&mut guid_data).unwrap();
            let mut guids = Vec::new();
            let mut stream = Cursor::new(guid_data);
            while let Ok(guid) = stream.read_le::<Guid>() {
                guids.push(guid);
            }

            guids
        };

        let user_strings = {
            let userstring_stream = physical_metadata
                .streams
                .iter()
                .find(|s| s.name == "#US")
                .expect("No #US stream found");
            let mut userstring_data = vec![0u8; userstring_stream.size as usize];
            c.seek(SeekFrom::Start(
                userstring_stream.offset as u64 + metadata_offset,
            ))
            .unwrap();
            c.read_exact(&mut userstring_data).unwrap();
            UserStringHeap::new(userstring_data)
        };

        let blobs = {
            let blob_stream = physical_metadata
                .streams
                .iter()
                .find(|s| s.name == "#Blob")
                .expect("No #Blob stream found");
            let mut blob_data = vec![0u8; blob_stream.size as usize];
            c.seek(SeekFrom::Start(blob_stream.offset as u64 + metadata_offset))
                .unwrap();
            c.read_exact(&mut blob_data).unwrap();
            BlobHeap::new(blob_data)
        };

        let mut r = Self {
            code_base,
            header: cli_header,
            guids,
            strings,
            user_strings,
            blobs,

            modules: vec![],
            type_refs: vec![],
            type_defs: vec![],
            fields: vec![],
            method_defs: vec![],
            params: vec![],
            member_refs: vec![],
            custom_attributes: vec![],
            stand_alone_sigs: vec![],
            assemblies: vec![],
            assembly_refs: vec![],
            interface_impls: vec![],
            constants: vec![],
            decl_security: vec![],
            class_layouts: vec![],
            field_layouts: vec![],
            property_maps: vec![],
            properties: vec![],
            method_semantics: vec![],
            method_impls: vec![],
            type_specs: vec![],
            impl_maps: vec![],
            nested_classes: vec![],
            generic_params: vec![],
            method_specs: vec![],
            generic_param_constraints: vec![],
            field_rvas: vec![],
        };

        let meta_streamheader = physical_metadata
            .streams
            .iter()
            .find(|s| s.name == "#~")
            .expect("No #~ stream found");
        let mut meta_data = vec![0u8; meta_streamheader.size as usize];
        c.set_position(meta_streamheader.offset as u64 + metadata_offset);
        c.read_exact(&mut meta_data)?;
        let mut meta_stream = Cursor::new(meta_data);
        let logical_metadata: crate::meta::LogicalMetadataTables = meta_stream.read_le().unwrap();
        let mut table = 0;
        for bit in 0..u64::BITS - 1 {
            if logical_metadata.valid & (1 << bit) != 0 {
                for _ in 0..logical_metadata.rows_per_table[table as usize] {
                    match bit {
                        0x00 => {
                            let module: tables::Module = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read Module table");
                            r.modules.push(module);
                        }
                        0x01 => {
                            let type_ref: tables::TypeRef = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read TypeRef table");
                            r.type_refs.push(type_ref);
                        }
                        0x02 => {
                            let type_def: tables::TypeDef = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read TypeDef table");
                            r.type_defs.push(type_def);
                        }
                        0x04 => {
                            let field: tables::Field = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read Field table");
                            r.fields.push(field);
                        }
                        0x06 => {
                            let method: tables::Method = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read Method table");
                            let (header, opcodes) =
                                parse_cil_bytecode(&method, r.code_base as u64, &mut c)?;
                            // parse_cil_method(&method, code_base as u64, &mut c, &userstring_heap)
                            //     .expect("Failed to parse CIL method");
                            r.method_defs.push((method, header, opcodes));
                        }
                        0x08 => {
                            let param: tables::Param = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read Param table");
                            r.params.push(param);
                        }
                        0x09 => {
                            let interface_impl: tables::InterfaceImpl = meta_stream
                                .read_le()
                                .expect("Failed to read InterfaceImpl table");
                            r.interface_impls.push(interface_impl);
                        }
                        0x0A => {
                            let member_ref: tables::MemberRef = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read MemberRef table");
                            r.member_refs.push(member_ref);
                        }
                        0x0B => {
                            let constant: tables::Constant = meta_stream
                                .read_le()
                                .expect("Failed to read Constant table");
                            r.constants.push(constant);
                        }
                        0x0C => {
                            let custom_attribute: tables::CustomAttribute = meta_stream
                                .read_le()
                                .expect("Failed to read CustomAttribute table");
                            r.custom_attributes.push(custom_attribute);
                        }
                        0x0D => {
                            return Result::Err(Error::UnsupportedTable("FieldMarshal"));
                        }
                        0x0E => {
                            let decl_security: tables::DeclSecurity = meta_stream
                                .read_le()
                                .expect("Failed to read DeclSecurity table");
                            r.decl_security.push(decl_security);
                        }
                        0x0F => {
                            let class_layout: tables::ClassLayout = meta_stream
                                .read_le()
                                .expect("Failed to read ClassLayout table");
                            r.class_layouts.push(class_layout);
                        }
                        0x10 => {
                            let field_layout: tables::FieldLayout = meta_stream
                                .read_le()
                                .expect("Failed to read FieldLayout table");
                            r.field_layouts.push(field_layout);
                        }
                        0x11 => {
                            let stand_alone_sig: tables::StandAloneSig = meta_stream
                                .read_le()
                                .expect("Failed to read StandaloneSig table");
                            r.stand_alone_sigs.push(stand_alone_sig);
                        }
                        0x12 => {
                            return Result::Err(Error::UnsupportedTable("EventMap"));
                        }
                        0x14 => {
                            return Result::Err(Error::UnsupportedTable("Event"));
                        }
                        0x15 => {
                            let property_map: tables::PropertyMap = meta_stream
                                .read_le()
                                .expect("Failed to read PropertyMap table");
                            r.property_maps.push(property_map);
                        }
                        0x17 => {
                            let property: tables::Property = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read Property table");
                            r.properties.push(property);
                        }
                        0x18 => {
                            let method_semantics: tables::MethodSemantics = meta_stream
                                .read_le()
                                .expect("Failed to read MethodSemantics table");
                            r.method_semantics.push(method_semantics);
                        }
                        0x19 => {
                            let method_impl: tables::MethodImpl = meta_stream
                                .read_le()
                                .expect("Failed to read MethodImpl table");
                            r.method_impls.push(method_impl);
                        }
                        0x1A => {
                            return Result::Err(Error::UnsupportedTable("ModuleRef"));
                        }
                        0x1B => {
                            let type_spec: tables::TypeSpec = meta_stream
                                .read_le()
                                .expect("Failed to read TypeSpec table");
                            r.type_specs.push(type_spec);
                        }
                        0x1C => {
                            let impl_map: tables::ImplMap = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read ImplMap table");
                            r.impl_maps.push(impl_map);
                        }
                        0x1D => {
                            let field_rva: tables::FieldRva = meta_stream
                                .read_le()
                                .expect("Failed to read FieldRVA table");
                            r.field_rvas.push(field_rva);
                        }
                        0x20 => {
                            let assembly: tables::Assembly = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read Assembly table");
                            r.assemblies.push(assembly);
                        }
                        0x21 => {
                            return Result::Err(Error::UnsupportedTable("AssemblyProcessor"));
                        }
                        0x22 => {
                            return Result::Err(Error::UnsupportedTable("AssemblyOS"));
                        }
                        0x23 => {
                            let assembly_ref: tables::AssemblyRef = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read AssemblyRef table");
                            r.assembly_refs.push(assembly_ref);
                        }
                        0x24 => {
                            return Result::Err(Error::UnsupportedTable("AssemblyRefProcessor"));
                        }
                        0x25 => {
                            return Result::Err(Error::UnsupportedTable("AssemblyRefOS"));
                        }
                        0x26 => {
                            return Result::Err(Error::UnsupportedTable("File"));
                        }
                        0x27 => {
                            return Result::Err(Error::UnsupportedTable("ExportedType"));
                        }
                        0x28 => {
                            return Result::Err(Error::UnsupportedTable("ManifestResource"));
                        }
                        0x29 => {
                            let nested_class: tables::NestedClass = meta_stream
                                .read_le()
                                .expect("Failed to read NestedClass table");
                            r.nested_classes.push(nested_class);
                        }
                        0x2A => {
                            let generic_param: tables::GenericParam = meta_stream
                                .read_le_args((&r.strings,))
                                .expect("Failed to read GenericParam table");
                            r.generic_params.push(generic_param);
                        }
                        0x2B => {
                            let method_spec: tables::MethodSpec = meta_stream
                                .read_le()
                                .expect("Failed to read MethodSpec table");
                            r.method_specs.push(method_spec);
                        }
                        0x2C => {
                            let generic_param_constraint: tables::GenericParamConstraint =
                                meta_stream
                                    .read_le()
                                    .expect("Failed to read GenericParamConstraint table");
                            r.generic_param_constraints.push(generic_param_constraint);
                        }
                        u => panic!("Invalid table: {:#X}", u),
                    }
                }
                table += 1;
            }
        }

        Ok(r)
    }

    fn parse_method_signature(&self, index: u16) -> Result<Option<StandaloneMethodSignature>> {
        let blob = self.blobs.get(index as u32).expect("Failed to get blob");
        let mut reader = Cursor::new(blob);
        Ok(Some(reader.read_le()?))
    }

    pub fn class_name(&self, index: u16) -> Result<Option<TypeName>> {
        let tdr = MemberRefParent::try_from(index as u32).expect("Failed to get MemberRefParent");
        Ok(tdr.typename(self))
    }

    pub fn resolve_method(
        &self,
        token: Token,
    ) -> Option<(TypeName, String, StandaloneMethodSignature)> {
        match token.kind() {
            TokenKind::MemberRef => {
                let member_ref = self.member_refs.get(token.index() as usize - 1).unwrap();
                let signature = self
                    .parse_method_signature(member_ref.signature_blob_index)
                    .expect("Failed to parse method signature")
                    .unwrap_or_default();
                let class_name = self.class_name(member_ref.class_index).unwrap().unwrap();
                Some((class_name, member_ref.name.clone(), signature))
            }
            TokenKind::MethodDef => {
                let (method_def, _, _) = self.method_defs.get(token.index() as usize - 1).unwrap();
                let signature = self
                    .parse_method_signature(method_def.signature_blob_index)
                    .expect("Failed to parse method signature")
                    .unwrap_or_default();
                Some((
                    TypeName {
                        namespace: "".to_string(),
                        name: "this".to_string(),
                    },
                    method_def.name.clone(),
                    signature,
                ))
            }
            u => {
                panic!("Invalid method token kind: {u:?}");
            }
        }
    }
}

pub struct MethodHeader {
    pub max_stack: u16,
    pub code_size: u32,
    pub local_var_sig_token: Option<Token>,
}

fn parse_cil_bytecode(
    def: &tables::Method,
    code_base: u64,
    code: &mut Cursor<&[u8]>,
) -> Result<(MethodHeader, Vec<(u32, RawOpcode)>)> {
    if def.flags.is_abstract() {
        println!(".method abstract {}() {{}}", def.name);
        return Ok((
            MethodHeader {
                max_stack: 0,
                code_size: 0,
                local_var_sig_token: None,
            },
            Vec::new(),
        ));
    }

    let header_start = def.rva as u64 - code_base;
    code.set_position(header_start);

    let header: u8 = code.read_le()?;
    let is_fat = header & 0x3 == 3;

    let header = if is_fat {
        code.set_position(header_start);
        let b: u16 = code.read_le()?;
        let _flags = b & 0xFFF;
        let size = b >> 12;

        let max_stack: u16 = code.read_le()?;
        let code_size: u32 = code.read_le()?;
        let local_var_sig_token: Token = code.read_le()?;

        code.set_position(header_start + (size * 4) as u64);

        MethodHeader {
            max_stack,
            code_size,
            local_var_sig_token: Some(local_var_sig_token),
        }
    } else {
        let size = header >> 2;
        MethodHeader {
            code_size: size as u32,
            max_stack: 8,
            local_var_sig_token: None,
        }
    };

    let mut data = vec![0u8; header.code_size as usize];
    code.read_exact(&mut data)?;

    let mut opcodes = Vec::new();
    let mut cil_cursor = Cursor::new(data);
    while cil_cursor.position() < cil_cursor.get_ref().len() as u64 {
        let offset = cil_cursor.position() as u32;
        let opcode: RawOpcode = cil_cursor.read_le()?;
        opcodes.push((offset, opcode));
    }

    Ok((header, opcodes))
}
