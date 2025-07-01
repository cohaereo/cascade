use binrw::BinReaderExt;
use chroma_dbg::ChromaDebug;
use cil::header::CliHeader;
use cil::meta::{PhysicalMetadata, Token};
use cil::opcodes::Opcode;
use cil::strings::StringHeap;
use cil::tables::{self, Method};
use object::pe::{IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR, ImageNtHeaders32, ImageNtHeaders64};
use object::read::pe::PeFile;
use object::{LittleEndian, Object, ObjectComdat, ObjectSection, ObjectSymbol};
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};

fn main() {
    let file = std::env::args().nth(1).expect("No file provided");
    let data = std::fs::read(file).expect("Failed to read file");

    let obj = PeFile::<'_, ImageNtHeaders32>::parse(&*data).expect("Failed to parse object file");
    let dir = obj
        .data_directories()
        .get(IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR)
        .expect("No COM descriptor directory found");

    let code_base = obj
        .nt_headers()
        .optional_header
        .base_of_code
        .get(LittleEndian);
    let cli_header_offset = dir.virtual_address.get(LittleEndian) - code_base;
    let code_section = obj
        .section_by_name(".text")
        .expect("No .text section found");
    let mut c = Cursor::new(code_section.data().unwrap());
    c.seek(SeekFrom::Start(cli_header_offset as u64)).unwrap();
    let cli_header: CliHeader = c.read_le().unwrap();
    println!("CLI Header: {:#?}", cli_header);
    println!("Code base: 0x{:X}", code_base);
    println!(
        "CLI physical metadata offset: 0x{:X}",
        cli_header.physical_metadata.rva - code_base
    );
    let metadata_offset = cli_header.physical_metadata.rva as u64 - code_base as u64;
    c.seek(SeekFrom::Start(metadata_offset)).unwrap();
    let physical_metadata: PhysicalMetadata = c.read_le().unwrap();
    println!("Physical Metadata: {:#?}", physical_metadata);

    let string_stream = physical_metadata
        .streams
        .iter()
        .find(|s| s.name == "#Strings")
        .expect("No #Strings stream found");

    let string_heap = {
        let mut string_data = vec![0u8; string_stream.size as usize];
        c.seek(SeekFrom::Start(
            string_stream.offset as u64 + metadata_offset,
        ))
        .unwrap();
        c.read_exact(&mut string_data).unwrap();
        StringHeap::new(string_data)
    };

    let guid_heap = {
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
        while let Ok(guid) = stream.read_le::<cil::meta::Guid>() {
            guids.push(guid);
        }

        guids
    };

    for s in physical_metadata.streams {
        println!("Stream: {:X?}", s);
        let mut stream_data = vec![0u8; s.size as usize];
        c.seek(SeekFrom::Start(s.offset as u64 + metadata_offset))
            .unwrap();
        c.read_exact(&mut stream_data).unwrap();
        let mut stream = Cursor::new(stream_data);

        match s.name.as_str() {
            "#~" => {
                let logical_metadata: cil::meta::LogicalMetadataTables = stream.read_le().unwrap();
                println!("Logical Metadata Tables: {:#X?}", logical_metadata);
                let mut table = 0;
                'tables: for bit in 0..u64::BITS - 1 {
                    if logical_metadata.valid & (1 << bit) != 0 {
                        println!(
                            "\nTable {}/{bit:02X}: {} rows",
                            table, logical_metadata.rows_per_table[table as usize]
                        );
                        for row in 0..logical_metadata.rows_per_table[table as usize] {
                            print!("Row {row:04}: ");
                            match bit {
                                0x00 => {
                                    let module: tables::Module = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read Module table");
                                    println!("{}", module.dbg_chroma());
                                }
                                0x01 => {
                                    let type_ref: tables::TypeRef = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read TypeRef table");
                                    println!("{}", type_ref.dbg_chroma());
                                }
                                0x02 => {
                                    let type_def: tables::TypeDef = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read TypeDef table");
                                    println!("{}", type_def.dbg_chroma());
                                }
                                0x04 => {
                                    let field: tables::Field = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read Field table");
                                    println!("{}", field.dbg_chroma());
                                }
                                0x06 => {
                                    let method: tables::Method = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read Method table");
                                    // println!("{}", method.dbg_chroma());
                                    parse_cil_method(&method, code_base as u64, &mut c)
                                        .expect("Failed to parse CIL method");
                                }
                                0x08 => {
                                    let param: tables::Param = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read Param table");
                                    println!("{}", param.dbg_chroma());
                                }
                                0x09 => {
                                    println!("TODO: InterfaceImpl");
                                    break 'tables;
                                }
                                0x0A => {
                                    let member_ref: tables::MemberRef = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read MemberRef table");
                                    println!("{}", member_ref.dbg_chroma());
                                }
                                0x0B => {
                                    println!("TODO: Constant");
                                    break 'tables;
                                }
                                0x0C => {
                                    let custom_attribute: tables::CustomAttribute = stream
                                        .read_le()
                                        .expect("Failed to read CustomAttribute table");
                                    println!("{}", custom_attribute.dbg_chroma());
                                }
                                0x0D => {
                                    println!("TODO: FieldMarshal");
                                    break 'tables;
                                }
                                0x0E => {
                                    println!("TODO: DeclSecurity");
                                    break 'tables;
                                }
                                0x0F => {
                                    println!("TODO: ClassLayout");
                                    break 'tables;
                                }
                                0x10 => {
                                    println!("TODO: FieldLayout");
                                    break 'tables;
                                }
                                0x11 => {
                                    let stand_alone_sig: tables::StandAloneSig = stream
                                        .read_le()
                                        .expect("Failed to read StandaloneSig table");
                                    println!("{}", stand_alone_sig.dbg_chroma());
                                }
                                0x12 => {
                                    println!("TODO: EventMap");
                                    break 'tables;
                                }
                                0x14 => {
                                    println!("TODO: Event");
                                    break 'tables;
                                }
                                0x15 => {
                                    println!("TODO: PropertyMap");
                                    break 'tables;
                                }
                                0x17 => {
                                    println!("TODO: Property");
                                    break 'tables;
                                }
                                0x18 => {
                                    println!("TODO: MethodSemantics");
                                    break 'tables;
                                }
                                0x19 => {
                                    println!("TODO: MethodImpl");
                                    break 'tables;
                                }
                                0x1A => {
                                    println!("TODO: ModuleRef");
                                    break 'tables;
                                }
                                0x1B => {
                                    println!("TODO: TypeSpec");
                                    break 'tables;
                                }
                                0x1C => {
                                    println!("TODO: ImplMap");
                                    break 'tables;
                                }
                                0x1D => {
                                    println!("TODO: FieldRVA");
                                    break 'tables;
                                }
                                0x20 => {
                                    let assembly: tables::Assembly = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read Assembly table");
                                    println!("{}", assembly.dbg_chroma());
                                }
                                0x21 => {
                                    println!("TODO: AssemblyProcessor");
                                    break 'tables;
                                }
                                0x22 => {
                                    println!("TODO: AssemblyOS");
                                    break 'tables;
                                }
                                0x23 => {
                                    let assembly_ref: tables::AssemblyRef = stream
                                        .read_le_args((&string_heap,))
                                        .expect("Failed to read AssemblyRef table");
                                    println!("{}", assembly_ref.dbg_chroma());
                                }
                                0x24 => {
                                    println!("TODO: AssemblyRefProcessor");
                                    break 'tables;
                                }
                                0x25 => {
                                    println!("TODO: AssemblyRefOS");
                                    break 'tables;
                                }
                                0x26 => {
                                    println!("TODO: File");
                                    break 'tables;
                                }
                                0x27 => {
                                    println!("TODO: ExportedType");
                                    break 'tables;
                                }
                                0x28 => {
                                    println!("TODO: ManifestResource");
                                    break 'tables;
                                }
                                0x29 => {
                                    println!("TODO: NestedClass");
                                    break 'tables;
                                }
                                0x2A => {
                                    println!("TODO: GenericParam");
                                    break 'tables;
                                }
                                0x2B => {
                                    println!("TODO: MethodSpec");
                                    break 'tables;
                                }
                                0x2C => {
                                    println!("TODO: GenericParamConstraint");
                                    break 'tables;
                                }
                                u => panic!("Invalid table: {:#X}", u),
                            }
                        }
                        table += 1;
                    }
                }

                println!("Tables ended at 0x{:X} in stream", stream.position());
            }
            "#Strings" => {}
            "#GUID" => {
                let mut i = 0;
                while let Ok(guid) = stream.read_le::<cil::meta::Guid>() {
                    println!("  GUID #{i}: {}", guid);
                    i += 1;
                }
            }
            u => {
                println!("Unhandled stream: {:?}", u);
            }
        }
    }

    // for section in obj.sections() {
    //     println!("Section: {}", section.name().unwrap_or("<unnamed>"));
    //     if let Ok(data) = section.data() {
    //         std::fs::write(
    //             format!("{}.dump", section.name().unwrap_or("<unnamed>")),
    //             data,
    //         );
    //     } else {
    //         println!("No data available for this section.");
    //     }
    // }
}

fn parse_cil_method(
    def: &Method,
    code_base: u64,
    code: &mut Cursor<&[u8]>,
) -> binrw::BinResult<()> {
    if def.flags.is_abstract() {
        println!("\n.method abstract {}()\n{{}}", def.name);
        return Ok(());
    }

    let header_start = def.rva as u64 - code_base;
    code.set_position(header_start);

    let header: u8 = code.read_le()?;
    let is_fat = header & 0x3 == 3;
    struct MethodHeader {
        max_stack: u16,
        code_size: u32,
    }

    let header = if is_fat {
        code.set_position(header_start);
        let b: u16 = code.read_le()?;
        let flags = b & 0xFFF;
        let size = b >> 12;

        let max_stack: u16 = code.read_le()?;
        let code_size: u32 = code.read_le()?;
        let local_var_sig_token: Token = code.read_le()?;

        code.set_position(header_start + (size * 4) as u64);

        MethodHeader {
            max_stack,
            code_size,
        }
    } else {
        let size = header >> 2;
        MethodHeader {
            code_size: size as u32,
            max_stack: 8,
        }
    };

    println!("\n.method {}()\n{{", def.name);
    println!("  .maxstack {}", header.max_stack);
    let mut data = vec![0u8; header.code_size as usize];
    code.read_exact(&mut data)?;

    let mut cil_cursor = Cursor::new(data);
    while cil_cursor.position() < cil_cursor.get_ref().len() as u64 {
        let pos = cil_cursor.position();
        let opcode: Opcode = cil_cursor.read_le()?;
        println!("  IL_{pos:04x}: {opcode}");
    }

    println!("}}");
    Ok(())
}
