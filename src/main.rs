use quatra::codegen::CodeGenerator;
use quatra::dna_assembly::DnaGenerator;
use quatra::interpreter::Interpreter;
use quatra::optimizer::Optimizer;
use quatra::parser::Parser;
use quatra::qir_translator::QirTranslator;
use quatra::qud::Qud;
use quatra::verilog;
use quatra::vm::ChimeraVM;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("QUATRA Compiler v0.1.0");
        eprintln!("Usage: quatra <command> [args]");
        eprintln!("Commands:");
        eprintln!("  compile <file.q>     Compile to ASM/Q-IR");
        eprintln!("  run <file.q>         Run with interpreter");
        eprintln!("  vm <file.q>          Execute with Chimera VM");
        eprintln!("  qir <file.q>         Show Q-IR");
        eprintln!("  dna <file.q>         Generate DNA sequence");
        eprintln!("  verilog <file.q>     Generate Verilog code");
        eprintln!("  native <file.q>      Compile with JIT (requires --features jit)");
        eprintln!("  repl                 Interactive REPL");
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "compile" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra compile <file.q>");
                std::process::exit(1);
            }
            let filepath = &args[2];
            match compile_file(filepath) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("Compilation error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra run <file.q>");
                std::process::exit(1);
            }
            let filepath = &args[2];
            match interpret_file(filepath) {
                Ok(result) => println!("Result: {:?}", result),
                Err(e) => {
                    eprintln!("Runtime error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "vm" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra vm <file.q>");
                std::process::exit(1);
            }
            let filepath = &args[2];
            match run_vm(filepath) {
                Ok(result) => println!("VM result: {:?}", result),
                Err(e) => {
                    eprintln!("VM error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "qir" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra qir <file.q>");
                std::process::exit(1);
            }
            let filepath = &args[2];
            match show_qir(filepath) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "dna" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra dna <file.q>");
                std::process::exit(1);
            }
            let filepath = &args[2];
            match generate_dna(filepath) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("DNA error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "verilog" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra verilog <file.q>");
                std::process::exit(1);
            }
            let filepath = &args[2];
            match generate_verilog_code(filepath) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("Verilog error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        "native" => {
            if args.len() < 3 {
                eprintln!("Usage: quatra native <file.q>");
                std::process::exit(1);
            }
            #[cfg(feature = "jit")]
            {
                let filepath = &args[2];
                match compile_native(filepath) {
                    Ok(bytes) => {
                        println!("Native code compiled: {} bytes", bytes.len());
                    }
                    Err(e) => {
                        eprintln!("JIT error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "jit"))]
            {
                eprintln!("JIT compilation requires --features jit");
                std::process::exit(1);
            }
        }

        "repl" => {
            println!("QUATRA REPL v0.1.0 (Ctrl+C to exit)");
            let mut interpreter = Interpreter::new();

            loop {
                use std::io::{self, Write};
                print!("quatra> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }
                if input == ":exit" || input == ":quit" {
                    break;
                }

                match interpreter.eval_str(input) {
                    Ok(result) => println!("=> {:?}", result),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Run 'quatra' without arguments to see available commands.");
            std::process::exit(1);
        }
    }
}

fn compile_file(filepath: &str) -> Result<String, String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);
    let mut codegen = CodeGenerator::new();
    Ok(codegen.generate(&optimized))
}

fn interpret_file(filepath: &str) -> Result<Qud, String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast.main)
}

fn run_vm(filepath: &str) -> Result<Qud, String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);

    let mut vm = ChimeraVM::new();
    vm.load_module(optimized);
    vm.run("main")
}

fn show_qir(filepath: &str) -> Result<(), String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;
    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);
    optimized.dump();
    Ok(())
}

fn generate_dna(filepath: &str) -> Result<String, String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);

    let mut generator = DnaGenerator::new();
    let sequences = generator.generate(&optimized);

    Ok(sequences
        .iter()
        .map(|s| s.to_genbank())
        .collect::<Vec<_>>()
        .join("\n"))
}

fn generate_verilog_code(filepath: &str) -> Result<String, String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);

    Ok(verilog::generate_verilog(&optimized))
}

#[cfg(feature = "jit")]
fn compile_native(filepath: &str) -> Result<Vec<u8>, String> {
    let source =
        fs::read_to_string(filepath).map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);

    quatra::jit::emit_native(&optimized)
}
