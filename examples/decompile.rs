use binrw::BinReaderExt;
use cascade::{Error, Result, opcodes::Opcode};
use std::{fmt::Write as _, io::Cursor};

use cil::{
    ReadExt,
    image::{CilImage, TypeName},
    opcodes::RawOpcode,
    signature::{self, Element, SignatureKind, StandaloneMethodSignature},
};

fn main() {
    let file = std::env::args().nth(1).expect("No file provided");
    let image = CilImage::load(file).expect("Failed to load CIL image");

    for (i, typedef) in image.type_defs.iter().enumerate() {
        println!(
            "type[{}] {}.{}",
            i, typedef.type_namespace, typedef.type_name
        );

        let method_start = typedef.method_list as usize - 1;
        let method_end = image
            .type_defs
            .get(i + 1)
            .map_or(image.method_defs.len(), |t| t.method_list as usize - 1);

        for (method, header, bytecode) in &image.method_defs[method_start..method_end] {
            let signature_blob = image
                .blobs
                .get(method.signature_blob_index as u32)
                .expect("Invalid method signature token");

            let signature = if signature_blob.is_empty() {
                None
            } else {
                Some(
                    StandaloneMethodSignature::parse(signature_blob)
                        .expect("Invalid method signature"),
                )
            };

            let mut locals = vec![];
            if let Some(standalone_token) = header.local_var_sig_token
                && standalone_token.index() > 0
            {
                let sig_token = &image.stand_alone_sigs[standalone_token.index() as usize - 1];
                let blob = image
                    .blobs
                    .get(sig_token.signature_blob_index as u32)
                    .expect("Invalid local var signature token");

                println!(
                    "// method {} sig={:?}",
                    method.name,
                    signature.as_ref().map(|s| s.debug_print(&image)),
                );

                let kind = SignatureKind::try_from(blob[0] & 0xF).expect("Invalid signature kind");
                assert_eq!(kind, SignatureKind::LocalVar);

                let mut c = Cursor::new(&blob[1..]);
                let count = c.read_compressed_u32().unwrap();
                for _ in 0..count {
                    match c.read_le::<signature::Element>() {
                        Ok(element) => {
                            locals.push(element);
                        }
                        Err(e) => {
                            eprintln!("Error reading local var element: {}", e);
                            break;
                        }
                    }
                }
            } else {
                println!("// method {}", method.name,);
            }
            // println!("  method {}", method.name);

            // for (j, instruction) in bytecode {
            //     print!("    IL_{j:04x}: ");
            //     match instruction {
            //         b if instruction.is_branch() => {
            //             let target = *j as i32 + b.branch_offset().unwrap();
            //             println!("{} IL_{target:04x}", b.asm_name());
            //         }
            //         m => println!("{m}"),
            //     }
            // }

            let decompiler = MethodDecompiler::new(&image, method, bytecode, &locals);

            match decompiler.decompile() {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Failed to decompile method: {}", e),
            }
        }

        // println!();
    }
}

#[must_use = "Call `decompile()` to get the decompiled output"]
struct MethodDecompiler<'img> {
    image: &'img CilImage,
    method: &'img cil::tables::Method,
    signature: StandaloneMethodSignature,
    bytecode: &'img [(u32, RawOpcode)],
    label_offsets: Vec<u32>,
    locals: &'img [signature::Element],

    stack: Stack,
}

impl<'img> MethodDecompiler<'img> {
    fn new(
        image: &'img CilImage,
        method: &'img cil::tables::Method,
        bytecode: &'img [(u32, RawOpcode)],
        locals: &'img [signature::Element],
    ) -> Self {
        let signature_blob = image
            .blobs
            .get(method.signature_blob_index as u32)
            .expect("Invalid method signature token");

        let signature = if signature_blob.is_empty() {
            None
        } else {
            Some(
                StandaloneMethodSignature::parse(signature_blob).expect("Invalid method signature"),
            )
        };

        let label_offsets = bytecode
            .iter()
            .filter_map(|(offset, opcode)| {
                if opcode.is_branch() {
                    Some((*offset as i32 + opcode.branch_offset().unwrap()) as u32)
                } else {
                    None
                }
            })
            .collect();

        Self {
            image,
            method,
            signature: signature.unwrap_or_default(),
            bytecode,
            label_offsets,
            locals,
            stack: Default::default(),
        }
    }

    /// Decompile the method to portable C++ code. Call `finish()` to get the output.
    pub fn decompile(mut self) -> cascade::Result<String> {
        let mut output = String::new();

        let mut temp_index = 0;

        let mut method_attributes = String::new();
        if self.method.flags.is_static() {
            method_attributes.push_str("static ");
        }

        writeln!(
            &mut output,
            "{method_attributes}{} {}({}) {{",
            self.signature.return_type.debug_print(self.image),
            self.method.name,
            self.signature
                .parameters
                .iter()
                .enumerate()
                .map(|(i, p)| format!("{} {}", p.debug_print(self.image), self.arg_var(i as u16)))
                .collect::<Vec<_>>()
                .join(", ")
        )?;

        for (i, local) in self.locals.iter().enumerate() {
            writeln!(
                &mut output,
                "    {} {}{{}};",
                local.debug_print(self.image),
                self.local_var(i as u16),
            )?;
        }
        if !self.locals.is_empty() {
            writeln!(&mut output)?;
        }

        for (op_offset, raw_opcode) in self.bytecode {
            if self.label_offsets.contains(op_offset) {
                writeln!(&mut output, "IL_{:04x}:", op_offset)?;
            }

            let opcode = Opcode::from(raw_opcode.clone());
            match opcode {
                Opcode::Nop {} => {}
                Opcode::LoadConstantI4(value) => {
                    self.stack.push(value.to_string());
                }
                Opcode::LoadLocal(index) => {
                    self.stack.push(self.local_var(index));
                }
                Opcode::StoreLocal(index) => {
                    writeln!(
                        &mut output,
                        "    {} = {};",
                        self.local_var(index),
                        self.stack.pop()?
                    )?;
                }
                Opcode::LoadLocalAddress(index) => {
                    self.stack.push(format!("&{}", self.local_var(index)));
                }
                Opcode::LoadArg(index) => {
                    self.stack.push(self.arg_var(index));
                }
                Opcode::LoadString(user_string) => {
                    self.stack.push(format!(
                        "\"{}\"",
                        self.image
                            .user_strings
                            .get(user_string)
                            .expect("Invalid user string index")
                    ));
                }
                Opcode::Add(ovf) => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({} + {})", left, right));
                }
                Opcode::Subtract(ovf) => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({} - {})", left, right));
                }
                Opcode::Multiply(ovf) => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({} * {})", left, right));
                }
                Opcode::Divide { unsigned } => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({} / {})", left, right));
                }
                Opcode::Remainder { unsigned } => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({} % {})", left, right));
                }
                Opcode::Compare {
                    comparison,
                    unsigned,
                } => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(comparison.operator(&left, Some(&right)));
                }
                Opcode::Return => {
                    if self.signature.return_type != Element::Void {
                        writeln!(&mut output, "    return {};", self.stack.pop()?)?;
                    } else {
                        writeln!(&mut output, "    return;")?;
                    }
                }
                Opcode::Call(t) => {
                    let (typename, methodname, signature) = self.image.resolve_method(t).unwrap();
                    let mut parameters: Vec<String> =
                        Vec::with_capacity(signature.parameters.len());

                    for _ in 0..signature.parameters.len() {
                        parameters.push(self.stack.pop().unwrap());
                    }

                    // Parameters are passed right-to-left
                    parameters.reverse();
                    if signature.header.has_this() {
                        parameters.insert(0, self.stack.pop().unwrap());
                    }

                    let mut method_path = self.translate_method_path(&typename, &methodname);
                    if method_path.starts_with("this::") {
                        method_path = method_path.replace("this::", "");
                    }
                    if signature.return_type == Element::Void {
                        write!(
                            &mut output,
                            "    {}({});",
                            method_path,
                            parameters.join(", ")
                        )?;
                    } else {
                        let result_temp = format!("temp{temp_index}");
                        temp_index += 1;
                        write!(
                            &mut output,
                            "    auto {} = {}({});",
                            result_temp,
                            method_path,
                            parameters.join(", ")
                        )?;

                        self.stack.push(result_temp);
                    }
                    writeln!(&mut output, " // {}", signature.debug_print(self.image))?;
                }
                Opcode::Branch(offset) => {
                    writeln!(
                        &mut output,
                        "    goto IL_{:04x};",
                        *op_offset as i32 + raw_opcode.size() as i32 + offset
                    )?;
                }
                Opcode::BranchConditional {
                    offset,
                    comparison,
                    unsigned,
                } => {
                    let expression = if comparison.is_true_false() {
                        comparison.operator(self.stack.pop()?, None)
                    } else {
                        comparison.operator(self.stack.pop()?, Some(self.stack.pop()?))
                    };
                    writeln!(
                        &mut output,
                        "    if ({expression}) goto IL_{:04x};",
                        *op_offset as i32 + raw_opcode.size() as i32 + offset
                    )?;
                }
                Opcode::ConvertToI1 => {
                    let top = self.stack.pop()?;
                    self.stack.push(format!("static_cast<int8>({})", top));
                }
                Opcode::ConvertToI2 => {
                    let top = self.stack.pop()?;
                    self.stack.push(format!("static_cast<int16>({})", top));
                }
                Opcode::ConvertToI4 => {
                    let top = self.stack.pop()?;
                    self.stack.push(format!("static_cast<int32>({})", top));
                }
                Opcode::ConvertToI8 => {
                    let top = self.stack.pop()?;
                    self.stack.push(format!("static_cast<int64>({})", top));
                }
                Opcode::ShiftLeft => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({left} << {right})"));
                }
                Opcode::ShiftRight => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({left} >> {right})"));
                }
                Opcode::Or => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({left} | {right})"));
                }
                Opcode::Xor => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({left} ^ {right})"));
                }
                Opcode::And => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    self.stack.push(format!("({left} & {right})"));
                }
                u => return Err(Error::UnimplementedOpcode(u)),
            }
        }

        writeln!(&mut output, "}}")?;

        Ok(output)
    }

    fn local_var(&self, index: u16) -> String {
        format!("var{index}")
    }

    fn arg_var(&self, index: u16) -> String {
        format!("arg{index}")
    }

    fn translate_method_path(&self, typename: &TypeName, path_cs: &str) -> String {
        if path_cs.ends_with(".ctor") {
            format!("{}::new", typename.path_cxx())
        } else {
            format!("{}::{}", typename.path_cxx(), path_cs.replace(".", "::"))
        }
    }
}

#[derive(Default)]
struct Stack {
    stack: Vec<String>,
}

impl Stack {
    fn push(&mut self, value: String) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<String> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }
}
