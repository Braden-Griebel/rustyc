use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use clap::{Parser};

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
    // CHANGE ME:
    // For now just print the ast
    let mut printer = parse::printing::Printer::new();
    printer.print_stmt(&program_ast);
    // For now just write an assembly file which returns 0
    _ = fs::write(
        cli.file.with_extension("s"),
        "    .globl main
main:
    movl    $0, %eax
    ret",
    );
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
