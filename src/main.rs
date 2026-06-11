use quatra::parser::Parser;
use quatra::interpreter::Interpreter;
use quatra::qir_translator::QirTranslator;
use quatra::optimizer::Optimizer;
use quatra::codegen::CodeGenerator;
use quatra::qud::Qud;
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
    let source = fs::read_to_string(filepath)
        .map_err(|e| format!("Failed to read {}: {}", filepath, e))?;

    let mut parser = Parser::new(&source)?;
    let ast = parser.parse_program()?;
    let translator = QirTranslator::new();
    let qir_module = translator.translate_program(&ast);
    let optimized = Optimizer::optimize(qir_module);
    let codegen = CodeGenerator::new();
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
