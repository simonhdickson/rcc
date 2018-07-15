use assembly;
use ast;
use lex;

pub fn compile(content: String, print_tokens: bool, print_ast: bool) -> String {
    let tokens = lex::parse(&content);

    if print_tokens {
        println!("{:#?}", tokens);
    }

    let program = ast::parse(tokens).unwrap();
    if print_ast {
        println!("{:#?}", program);
    }

    assembly::generate(program)
}
