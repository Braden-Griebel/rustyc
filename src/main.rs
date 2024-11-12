use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use clap::{Parser};
use crate::assemble::emmiting::Emitter;

mod lex;
mod parse;
mod assemble;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to operate on
    file: PathBuf,

    /// Run the lex, but stop before parsing
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    lex: bool,

    /// Run the lex and parse, but stop before assembly generation
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    parse: bool,

    /// Perform lexing, parsing, and assembly generation, but stop before code emission
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    codegen: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    // Preprocess source file
    let _response = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(cli.file.clone())
        .arg("-o")
        .arg(cli.file.with_extension("i"))
        .output()
        .expect("Failed to preprocess file");

    // Read in the preprocessed file and compile it to assembly
    let source_file =
        fs::read_to_string(cli.file.with_extension("i")).expect("Unable to read preprocessed file");
    // Delete preprocessed file
    _ = fs::remove_file(cli.file.with_extension("i"));
    // COMPILE
    // Lex the source file
    let mut lexer = lex::Lexer::new(source_file);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_) => return ExitCode::FAILURE,
    };
    if cli.lex {
        return ExitCode::SUCCESS;
    }
    // Compile the source file to an AST
    let mut parser = parse::parsing::Parser::new(tokens);
    let program_ast = match parser.parse() {
        Ok(program_ast) => program_ast,
        Err(_) => return ExitCode::FAILURE,
    };
    if cli.parse {
        return ExitCode::SUCCESS;
    }
    // Assemble the c_ast into an assembly ast
    let assembler = assemble::assembling::Assembler::new();
    let assembly_ast = match assembler.assemble(program_ast){
        Ok(assembly_ast) => assembly_ast,
        Err(_) => return ExitCode::FAILURE,
    };
    // Emit the assembly to a file
    let mut emitter = Emitter::new();
    match emitter.emit(cli.file.with_extension("s"), assembly_ast){
        Ok(_) => (),
        Err(_) => return ExitCode::FAILURE,
    };
    // Link the assembly file
    _ = Command::new("gcc")
        .arg(cli.file.with_extension("s"))
        .arg("-o")
        .arg(cli.file.with_extension(""))
        .output()
        .expect("Unable to link assembly file");
    // Delete the assembly file
    _ = fs::remove_file(cli.file.with_extension("s"));
    // If succesful, return 0
    ExitCode::from(0)
}
