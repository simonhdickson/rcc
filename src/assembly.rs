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
        Statement::Return(expression) => {
            let expression = generate_expression(expression);
            format!(r"{}
 ret", expression)
        }
    }
}

fn generate_expression(expression: Expression) -> String {
    match expression {
        Expression::Constant(i) => {
            format!(r" movl    ${}, %eax", i)
        },
        Expression::UnaryOperation(Operator::Negation, expression) => {
            format!(r"{}
 neg     %eax", generate_expression(*expression))
        },
        Expression::UnaryOperation(Operator::Complement, expression) => {
            format!(r"{}
 not     %eax", generate_expression(*expression))
        },
        Expression::UnaryOperation(Operator::LogicalNegation, expression) => {
            format!(r"{}
 cmpl   $0, %eax
 movl   $0, %eax
 sete   %al", generate_expression(*expression))
        },
    }
}
