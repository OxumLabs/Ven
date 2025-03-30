use std::env;
use std::fs;
use std::process::exit;
use std::path::Path;

use optimisers::pass1::optimize_pass1;
use optimisers::pass2::pass2;
use parse::AST;
use token::Tokenizer;
use errmsgs::print_errors;
use transpilers::C::transpile_c;
use transpilers::LLVM::transpile_llvm;
use transpilers::LX8664::transpile_lx8664;
use transpilers::RST::transpile_rs;

pub mod errs;
pub mod parse;
pub mod token;
pub mod var_checker;
pub mod errmsgs;
pub mod optimisers;
pub mod parse1;
pub mod transpilers;

fn print_help() {
    println!("Ven Engine");
    println!("├── --in=<file_path.ven>   Input file (must end with .ven)");
    println!("├── -t=<rs/rust,c,llvm>           Target output format");
    println!("├── -h, --help             Show help information");
    println!("└── -v, --version, --ver   Show version information");
}

fn print_version() {
    println!("Ven Engine Version 0.0.1");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        exit(1);
    }

    let mut input_file: Option<String> = None;
    let mut target: Option<String> = None;

    for arg in &args[1..] {
        if arg == "-h" || arg == "--help" {
            print_help();
            exit(0);
        } else if arg == "-v" || arg == "--version" || arg == "--ver" {
            print_version();
            exit(0);
        } else if arg.starts_with("--in=") {
            input_file = Some(arg.trim_start_matches("--in=").to_string());
        } else if arg.starts_with("-t=") || arg.starts_with("--target=") {
            target = Some(arg.trim_start_matches("-t=").trim_start_matches("--target=").to_string());
        }
    }

    let input_path = match input_file {
        Some(path) if path.ends_with(".ven") => path,
        _ => {
            eprintln!("Error: Input file must be specified with --in=<file_path.ven> and end with .ven");
            exit(1);
        }
    };

    let target_lang = match target.as_deref() {
        Some("rs") | Some("rust") => "rust",
        Some("c") | Some("C") => "c",
        Some("lx8664") | Some("LX8664") => "lx8664",
        Some("llvm") | Some("LLVM") => "llvm",
        Some(t) => {
            eprintln!("Error: Unsupported target '{}'", t);
            exit(1);
        }
        None => {
            eprintln!("Error: Missing -t=<target> argument (e.g., -t=rs)");
            exit(1);
        }
    };

    let code = match fs::read_to_string(&input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", input_path, e);
            exit(1);
        }
    };

    let mut tokenizer = Tokenizer::new(&code);
    tokenizer.tokenize();

    let (mut ast, _var_map, errors) = AST::parse(&tokenizer.tokens, &code);

    if !errors.is_empty() {
        print_errors(&errors);
        exit(1);
    }
    optimize_pass1(&mut ast);
    let ast = pass2(ast);
    ast.debug();

    if target_lang == "rust" {
        let rust_code = transpile_rs(&ast);

        let output_path = Path::new(&input_path).with_extension("rs");
        match fs::write(&output_path, rust_code) {
            Ok(_) => println!("Successfully transpiled to Rust: {}", output_path.display()),
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    }
    else if target_lang == "c" {
        let rust_code = transpile_c(&ast);

        let output_path = Path::new(&input_path).with_extension("c");
        match fs::write(&output_path, rust_code) {
            Ok(_) => println!("Successfully transpiled to C: {}", output_path.display()),
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    }
    else if target_lang == "llvm" {
        let rust_code = transpile_llvm(&ast);

        let output_path = Path::new(&input_path).with_extension("ll");
        match fs::write(&output_path, rust_code) {
            Ok(_) => println!("Successfully transpiled to LLVM_IR: {}", output_path.display()),
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    }
    else if target_lang == "lx8664" {
        let rust_code = transpile_lx8664(&ast);
        let output_path = Path::new(&input_path).with_extension("asm");
        match fs::write(&output_path, rust_code) {
            Ok(_) => println!("Successfully transpiled to x86_64 Assembly: {}", output_path.display()),
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    }
    
}
