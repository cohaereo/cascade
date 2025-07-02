use cil::image::CilImage;

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

        for (method, bytecode) in &image.methods[method_start..method_end] {
            println!("  method {}", method.name);

            for (j, instruction) in bytecode {
                println!("    IL{:04x}: {}", j, instruction);
            }
        }

        println!();
    }
}
