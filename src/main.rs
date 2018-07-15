extern crate clap;
#[macro_use] extern crate failure;

mod assembly;
mod ast;
mod lex;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};

fn main() {
    let matches =
        App::new("rcc")
            .version("0.0.1")
            .author("Simon Dickson <simon@simonhdickson.com>")
            .about("C compiler written in Rust, because why not?")
            .arg(Arg::with_name("SOURCE")
                .help("Sets the source file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("print-tokens")
                .long("print-tokens")
                .help("Prints tokens"))
            .arg(Arg::with_name("print-ast")
                .long("print-ast")
                .help("Prints AST"))
            .get_matches();

    let source_file = matches.value_of("SOURCE").unwrap();
    let assembly_file = source_file.replace(".c", ".s");
    let prints_tokens = matches.is_present("print-tokens");
    let print_ast = matches.is_present("print-ast");

    let mut f_in = File::open(source_file).unwrap();
    let mut f_out = File::create(assembly_file).unwrap();

    let mut content = String::new();
    f_in.read_to_string(&mut content).expect("something went wrong reading the file");

    let tokens = lex::parse(&content);

    if prints_tokens {
        println!("{:?}", tokens);
    }

    match ast::parse(tokens) {
        Ok (program) => {
            if print_ast {
                println!("{:?}", program);
            }

            let content = assembly::generate(program);

            f_out.write_all(content.as_bytes()).expect("something went wrong writing the file");
        },
        Err(err) => {
            println!("{}", err)
        }
    }
}
