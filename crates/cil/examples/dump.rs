use binrw::BinReaderExt;
use std::io::Cursor;

use cil::{
    ReadExt,
    image::CilImage,
    signature::{self, SignatureKind},
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
            .map_or(image.methods.len(), |t| t.method_list as usize - 1);

        for (method, header, bytecode) in &image.methods[method_start..method_end] {
            if let Some(standalone_token) = header.local_var_sig_token {
                let sig_token = &image.stand_alone_sigs[standalone_token.index() as usize - 1];
                let blob = image
                    .blobs
                    .get(sig_token.signature_blob_index as u32)
                    .expect("Invalid local var signature token");

                println!(
                    "  method {} (local var sig {standalone_token:?}: {:?})",
                    method.name, blob
                );

                let kind = SignatureKind::try_from(blob[0] & 0xF).expect("Invalid signature kind");
                assert_eq!(kind, SignatureKind::LocalVar);

                let mut c = Cursor::new(&blob[1..]);
                let count = c.read_compressed_u32().unwrap();
                println!("    .locals init (");
                for i in 0..count {
                    let element: signature::Element = c.read_le().unwrap();
                    println!("        [{i}] {},", element);
                }
                println!("    )");
            } else {
                continue;
            }
            println!("  method {}", method.name);

            // for (j, instruction) in bytecode {
            //     println!("    IL{:04x}: {}", j, instruction);
            // }
        }

        println!();
    }
}
