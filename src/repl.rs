use std::{
    io::{self, Write, stdin, stdout},
    fs::{self, File},
    process::{exit, Command},
    env::consts::OS,
};

use crate::{parse::pven, mkasm};

pub struct Repl {
    input: String,
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            stdin: stdin(),
            stdout: stdout(),
        }
    }

    pub fn start(&mut self) {
        println!("Ven IR Language REPL v0.1.0");
        println!("Type 'exit' to quit\n");

        loop {
            // Print prompt and flush stdout
            print!(">>> ");
            self.stdout.flush().unwrap();

            // Read user input
            self.input.clear();
            if self.stdin.read_line(&mut self.input).unwrap() == 0 {
                // EOF reached
                break;
            }

            let input = self.input.trim();
            if input.is_empty() {
                continue;
            }

            if input == "exit" {
                break;
            }

            self.execute_input(input);
        }
    }

    fn execute_input(&self, input: &str) {
        // Parse and execute the input
        match pven(input.to_string()) {
            tokens => {
                // Use l64 (Linux 64-bit) as default target for REPL
                let asm = mkasm::mkasm(tokens, "LM".to_string());
                
                // Create temporary assembly file
                let asm_file_path = ".repl.asm";
                let object_file_path = ".repl.o";

                if let Ok(mut asmf) = File::create(asm_file_path) {
                    if let Err(e) = asmf.write_all(asm.as_bytes()) {
                        eprintln!("Error writing assembly: {}", e);
                        return;
                    }

                    // Compile and run
                    if self.compile_and_run(asm_file_path, object_file_path) {
                        // Clean up temporary files
                        let _ = fs::remove_file(asm_file_path);
                        let _ = fs::remove_file(object_file_path);
                        let _ = fs::remove_file("a.out");
                    }
                }
            }
        }
    }

    fn compile_and_run(&self, asm_file: &str, object_file: &str) -> bool {
        let nasm_cmd = match OS {
            "linux" | "macos" => "nasm",
            "windows" => "./nasm.exe",
            _ => {
                eprintln!("Unsupported Platform!");
                return false;
            }
        };

        // Use the same nasm arguments as in main.rs
        let nasmargs = ["-f", "elf64", asm_file, "-o", object_file];

        if !Command::new(nasm_cmd)
            .args(&nasmargs)
            .status()
            .map_or(false, |s| s.success())
        {
            eprintln!("Assembly failed! Ensure NASM is installed.");
            return false;
        }

        // Use the same linker detection as in main.rs
        let linker = if Command::new("ld.lld").arg("--version").output().is_ok() {
            "ld.lld"
        } else {
            "ld"
        };

        let linker_args = ["-o", "a.out", object_file, "-no-pie"];

        if !Command::new(linker)
            .args(&linker_args)
            .status()
            .map_or(false, |s| s.success())
        {
            eprintln!("Linking failed! Ensure the linker is installed.");
            return false;
        }

        // Execute the compiled program
        if let Ok(output) = Command::new("./a.out").output() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            io::stdout().flush().unwrap();
            true
        } else {
            eprintln!("Failed to execute program!");
            false
        }
    }
} 
