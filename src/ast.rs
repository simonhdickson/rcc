use std::iter::Iterator;

use failure::Error;
use lex::Token;

#[derive(Debug)]
pub struct Program {
    pub function: Function
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression)
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32)
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut token_iter = tokens.into_iter();
    
    parse_program(&mut token_iter)
}

fn parse_program(mut tokens: &mut Iterator<Item = Token>) -> Result<Program, Error> {
    match parse_function(&mut tokens) {
        Ok(ref fun) if fun.name != "main" => Err(format_err!("expected main function")),
        Ok(function) => Ok(Program { function }),
        Err(err) => Err(err),
    }
}

fn parse_function(tokens: &mut Iterator<Item = Token>) -> Result<Function, Error> {
    match tokens.next() {
        Some(Token::Integer) => Ok(()),
        _ => Err(format_err!("expected return type")),
    }?;

    let name =
        match tokens.next() {
            Some(Token::Ident(name)) => Ok(name),
            _ => Err(format_err!("expected function name")),
        }?;

    if Some(Token::LeftParenthesis) != tokens.next() || Some(Token::RightParenthesis) != tokens.next() {
         return Err(format_err!("expected parameter list"));
    }

    if Some(Token::LeftBrace) != tokens.next() {
         return Err(format_err!("expected opening brace"));
    }

    let statement = parse_statement(tokens)?;

    if Some(Token::RightBrace) != tokens.next() {
         return Err(format_err!("expected closing brace"));
    }

    Ok(Function { name: name.to_owned(), statement })
}

fn parse_statement<'a>(tokens: &mut Iterator<Item = Token>) -> Result<Statement, Error> {
    match tokens.next() {
        Some(Token::Return) => Ok(()),
        _ => Err(format_err!("expected return statement")),
    }?;

    let expression = parse_expression(tokens)?;

    if Some(Token::SemiColon) != tokens.next() {
         return Err(format_err!("expected semi colon"));
    }
    
    Ok(Statement::Return(expression))
}

fn parse_expression<'a>(tokens: &mut Iterator<Item = Token>) -> Result<Expression, Error> {
    let i =
        match tokens.next() {
            Some(Token::LiteralInteger(i)) => Ok(i.parse::<i32>()?),
            _ => Err(format_err!("expected integer literal")),
        }?;
    
    Ok(Expression::Constant(i))
}
