use std::{env::args, fs::read_to_string, process::exit};

use parse::pven;
pub mod archs;
pub mod mkasm;
pub mod parse;
pub mod types;

#[allow(unused)]
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        print_usage("Not enough arguments provided!");
        exit(1);
    }

    let file = &args[1];

    if !file.ends_with(".ven") {
        print_usage(&format!("File type not supported! Given file: {}", file));
        exit(1);
    }

    match read_to_string(&file) {
        Ok(venc) => {
            match pven(venc) {
                (pc) => {
                    let target = match args[2].as_str() {
                        "l64" => "LM".to_string(),
                        "l32" => "LHM".to_string(),
                        "w32" => "WHM".to_string(),
                        "w64" => "WM".to_string(),
                        "m64" => "MM".to_string(),
                        "m32" => "MHM".to_string(),
                        _ => {
                            print_usage("Invalid build target! Use 'l64', 'l32', 'w64', 'w32', 'm64', or 'm32'.");
                            exit(1);
                        }
                    };

                    let asm = mkasm::mkasm(pc, target);
                    println!("ASM CODE ->\n{}", asm);
                }
            }
        }
        Err(e) => {
            print_usage(&format!("Unable to read the input file! Error: {}", e));
            exit(1);
        }
    }
}

fn print_usage(error_message: &str) {
    eprintln!("Error: {}\n", error_message);
    println!("Usage: <program_name> <file.ven> <target>");
    println!("Where:");
    println!("  <file.ven>   Path to the input file with .ven extension.");
    println!("  <target>     Build target:");
    println!("                'l64' - for 64-bit Linux");
    println!("                'l32' - for 32-bit Linux");
    println!("                'w64' - for 64-bit Windows");
    println!("                'w32' - for 32-bit Windows");
    println!("                'm64' - for 64-bit macOS");
    println!("                'm32' - for 32-bit macOS");
    println!();
    println!("Example:");
    println!("  <program_name> example.ven l64");
    println!();
    println!("Please ensure that the input file has the correct extension and that the target is valid.");
}
