extern crate clap;
#[macro_use] extern crate failure;

mod assembly;
mod ast;
mod cli;
mod compiler;
mod lex;
mod lib;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = cli::build_cli().get_matches();

    let source_file = matches.value_of("SOURCE").unwrap();
    let assembly_file = Path::new(source_file).file_name().unwrap().to_str().unwrap().replace(".c", ".s");
    let print_tokens = matches.is_present("print-tokens");
    let print_ast = matches.is_present("print-ast");

    let mut f_in = File::open(source_file).unwrap();
    let mut f_out = File::create(assembly_file).unwrap();

    let mut content = String::new();
    f_in.read_to_string(&mut content).expect("something went wrong reading the file");

    let content = compiler::compile(content, print_tokens, print_ast);

    f_out.write_all(content.as_bytes()).expect("something went wrong writing the file");
}
