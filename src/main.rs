use std::env;
use std::fs;
use std::process::exit;
use std::path::Path;
use std::time::{Duration, Instant};

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

/// Formats a Duration into a human-readable string using µs, ms, s, or min.
fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();
    if micros < 1_000 {
        return format!("{} µs", micros);
    }
    let millis = duration.as_millis();
    if millis < 1_000 {
        return format!("{} ms", millis);
    }
    let secs = duration.as_secs_f64();
    if secs < 60.0 {
        return format!("{:.2} s", secs);
    }
    let mins = secs / 60.0;
    format!("{:.2} min", mins)
}

fn print_help() {
    println!("Ven Engine");
    println!("├── --in=<file_path.ven>         Input file (must end with .ven)");
    println!("├── -t=<rs/rust,c,llvm,lx8664>     Target output format");
    println!("├── --show-msgs or -sm            Show messages in a tree-like view");
    println!("├── -h, --help                   Show help information");
    println!("└── -v, --version, --ver         Show version information");
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
    let mut show_msgs = false;

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
        } else if arg == "--show-msgs" || arg == "-sm" {
            show_msgs = true;
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

    if show_msgs {
        println!("Running:");
    }

    // --- Tokenizing ---
    let start = Instant::now();
    let mut tokenizer = Tokenizer::new(&code);
    tokenizer.tokenize();
    let token_time = start.elapsed();
    if show_msgs {
        println!("├── Tokenizing... took {}", format_duration(token_time));
    }

    // --- Parsing AST ---
    let start = Instant::now();
    let (mut ast, _var_map, errors) = AST::parse(&tokenizer.tokens, &code);
    let parse_time = start.elapsed();
    if show_msgs {
        println!("├── Parsing AST... took {}", format_duration(parse_time));
    }
    if !errors.is_empty() {
        print_errors(&errors);
        exit(1);
    }

    // --- Optimizing AST (pass1) ---
    let start = Instant::now();
    optimize_pass1(&mut ast);
    let opt1_time = start.elapsed();
    if show_msgs {
        println!("├── Optimizing AST (pass1)... took {}", format_duration(opt1_time));
    }

    // --- Optimizing AST (pass2) ---
    let start = Instant::now();
    let ast = pass2(ast);
    let opt2_time = start.elapsed();
    if show_msgs {
        println!("├── Optimizing AST (pass2)... took {}", format_duration(opt2_time));
    }

    // --- Transpiling ---
    if show_msgs {
        println!("├── Transpiling to {}...", target_lang);
    }
    if target_lang == "rust" {
        let start = Instant::now();
        let rust_code = transpile_rs(&ast);
        let transp_time = start.elapsed();
        let output_path = Path::new(&input_path).with_extension("rs");
        match fs::write(&output_path, rust_code) {
            Ok(_) => {
                if show_msgs {
                    println!("└── Transpiling to Rust... took {}", format_duration(transp_time));
                }
                println!("Successfully transpiled to Rust: {}", output_path.display());
            },
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    } else if target_lang == "c" {
        let start = Instant::now();
        let c_code = transpile_c(&ast);
        let transp_time = start.elapsed();
        let output_path = Path::new(&input_path).with_extension("c");
        match fs::write(&output_path, c_code) {
            Ok(_) => {
                if show_msgs {
                    println!("└── Transpiling to C... took {}", format_duration(transp_time));
                }
                println!("Successfully transpiled to C: {}", output_path.display());
            },
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    } else if target_lang == "llvm" {
        let start = Instant::now();
        let llvm_code = transpile_llvm(&ast);
        let transp_time = start.elapsed();
        let output_path = Path::new(&input_path).with_extension("ll");
        match fs::write(&output_path, llvm_code) {
            Ok(_) => {
                if show_msgs {
                    println!("└── Transpiling to LLVM_IR... took {}", format_duration(transp_time));
                }
                println!("Successfully transpiled to LLVM_IR: {}", output_path.display());
            },
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    } else if target_lang == "lx8664" {
        let start = Instant::now();
        let asm_code = transpile_lx8664(&ast);
        let transp_time = start.elapsed();
        let output_path = Path::new(&input_path).with_extension("asm");
        match fs::write(&output_path, asm_code) {
            Ok(_) => {
                if show_msgs {
                    println!("└── Transpiling to x86_64 Assembly... took {}", format_duration(transp_time));
                }
                println!("Successfully transpiled to x86_64 Assembly: {}", output_path.display());
            },
            Err(e) => {
                eprintln!("Error writing to file {}: {}", output_path.display(), e);
                exit(1);
            }
        }
    }
}
