use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
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
}
