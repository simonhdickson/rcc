use ast::*;

pub fn generate(program: Program) -> String {
    match program {
        Program { function } => generate_function(function)
    }
}

fn generate_function(function: Function) -> String {
    match function {
        Function { name, statement } => {
            format!(r" .globl {}
{}:
{}", name, name, generate_statement(statement))
        }
    }
}

fn generate_statement(statement: Statement) -> String {
    match statement {
        Statement::Return(Expression::Constant(i)) => {
            format!(r" movl    ${}, %eax
 ret", i)
        }
    }
}
