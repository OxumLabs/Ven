use std::{
    env::{args, consts::OS},
    fs::{self, read_to_string, File},
    io::Write,
    process::{exit, Command},
};

use parse::pven;
pub mod archs;
pub mod mkasm;
pub mod parse;
pub mod types;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 || args.len() > 4 {
        print_usage("Invalid number of arguments provided!");
        exit(1);
    }

    let file = &args[1];
    if !file.ends_with(".ven") {
        print_usage(&format!("File type not supported! Given file: {}", file));
        exit(1);
    }

    let retain_asm = args.get(3).map_or(false, |arg| arg == "--retain-asm");

    let custom_linker = args.iter().find_map(|arg| {
        if arg.starts_with("--linker=") {
            Some(arg.split('=').nth(1).unwrap())
        } else {
            None
        }
    });

    match read_to_string(&file) {
        Ok(venc) => match pven(venc) {
            pc => {
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

                let asm = mkasm::mkasm(pc, target.clone());
                println!("asm for target '{}':\n{}",target,asm);
                let asm_file_path = "a.asm";

                match File::create(asm_file_path) {
                    Ok(mut asmf) => {
                        if let Err(e) = asmf.write_all(asm.as_bytes()) {
                            eprintln!("Unable to write assembly! Error: {}", e);
                            exit(1);
                        }

                        let object_file_path = "a.o";
                        if compile_target(args[2].as_str(), object_file_path, custom_linker.as_deref()) {
                            clean_up(retain_asm, asm_file_path, object_file_path);
                        } else {
                            exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Unable to create assembly file! Error: {}", e);
                        exit(1);
                    }
                }
            }
        },
        Err(e) => {
            print_usage(&format!("Unable to read the input file! Error: {}", e));
            exit(1);
        }
    }
}

fn compile_target(target: &str, object_file: &str, custom_linker: Option<&str>) -> bool {
    let nasm_cmd = match OS {
        "linux" => "nasm",
        "windows" => "./nasm.exe",
        "macos" => "nasm",
        _ => {
            eprintln!("Unsupported Platform!");
            exit(1);
        }
    };

    let nasmargs = match target {
        "l64" => vec!["-f", "elf64", "a.asm", "-o", object_file],
        "l32" => vec!["-f", "elf32", "a.asm", "-o", object_file],
        "w64" => vec!["-f", "win64", "a.asm", "-o", object_file],
        "w32" => vec!["-f", "win32", "a.asm", "-o", object_file],
        "m64" => vec!["-f", "macho64", "a.asm", "-o", object_file],
        "m32" => vec!["-f", "macho32", "a.asm", "-o", object_file],
        _ => {
            eprintln!("Unsupported target: {}", target);
            return false;
        }
    };

    if !Command::new(nasm_cmd)
        .args(&nasmargs)
        .status()
        .map_or(false, |s| s.success())
    {
        eprintln!("Assembly failed! Ensure NASM is installed.");
        return false;
    }

    let linker = custom_linker.unwrap_or_else(|| {
        match target {
            "w64" | "w32" => {
                if Command::new("lld-link").arg("--version").output().is_ok() {
                    "lld-link"
                } else if Command::new("tdm-gcc").arg("--version").output().is_ok() {
                    "tdm-gcc"
                } else {
                    "link"
                }
            },
            "m64" | "m32" => "ld",
            "l64" | "l32" => {
                if Command::new("ld.lld").arg("--version").output().is_ok() {
                    "ld.lld"
                } else {
                    "ld"
                }
            },
            _ => "ld",
        }
    });

    let linker_args = match linker {
        "lld-link" => vec!["/OUT:a.exe", "/SUBSYSTEM:CONSOLE", object_file],
        "tdm-gcc" => vec!["/OUT:a.exe", "/SUBSYSTEM:CONSOLE", object_file],
        "link" => vec!["/OUT:a.exe", object_file],
        "ld.lld" => match target {
            "l64" => vec!["-o", "a.out", object_file, "-no-pie"],
            "l32" => vec!["-m", "elf_i386", "-o", "a.out", object_file],
            _ => unreachable!(),
        },
        "ld" => match target {
            "m64" | "m32" => vec!["-o", "a.out", object_file],
            _ => unreachable!(),
        },
        _ => {
            eprintln!("Unsupported linker: {}", linker);
            return false;
        }
    };

    if !Command::new(linker)
        .args(&linker_args)
        .status()
        .map_or(false, |s| s.success())
    {
        eprintln!("Linking failed! Ensure the linker is installed.");
        return false;
    }

    true
}

fn clean_up(retain: bool, asm_file: &str, object_file: &str) {
    if !retain {
        let _ = fs::remove_file(asm_file);
        let _ = fs::remove_file(object_file);
    } else {
        let _ = fs::remove_file(object_file);
    }
}

fn print_usage(msg: &str) {
    eprintln!("{}", msg);
    eprintln!("Usage: ven [input_file.ven] [target] [--retain-asm]");
    eprintln!("Targets: l64, l32, w64, w32, m64, m32");
}
