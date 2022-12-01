mod error;

use error::*;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison operations
    Eql,
    Neql,
    Gt,
    Lt,
    Ge,
    Le,
    Max,
    Min,
    // Logical operations
    And,
    Or,
    Not,
    // Number and variables
    Number(f64),
    Bool(bool),
}

fn tokenize(code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    code.chars()
        .map(|c| match c {
            '(' => " ( ".to_string(),
            ')' => " ) ".to_string(),
            c => c.to_string(),
        })
        .collect::<String>()
        .split_whitespace()
        .map(|token| {
            Ok(match token {
                "(" => Token::OpenBracket,
                ")" => Token::CloseBracket,
                "+" => Token::Add,
                "-" => Token::Sub,
                "*" => Token::Mul,
                "/" => Token::Div,
                "%" => Token::Mod,
                "=" => Token::Eql,
                "!=" => Token::Neql,
                ">" => Token::Gt,
                "<" => Token::Lt,
                ">=" => Token::Ge,
                "<=" => Token::Le,
                "max" => Token::Max,
                "min" => Token::Min,
                "and" => Token::And,
                "or" => Token::Or,
                "not" => Token::Not,
                token => {
                    let parse = token.parse::<f64>()?;
                    Token::Number(parse)
                }
            })
        })
        .collect()
}

pub fn eval(code: &str) -> Result<Vec<Vec<Token>>, Box<dyn Error>> {
    let stack: Vec<Token> = tokenize(code)?;

    let mut process_stack: Vec<Vec<Token>> = Vec::new();

    for (i, token) in stack.into_iter().enumerate() {
        match token {
            Token::OpenBracket => process_stack.push(Vec::new()),
            Token::Add
            | Token::Sub
            | Token::Mul
            | Token::Div
            | Token::Mod
            | Token::Eql
            | Token::Neql
            | Token::Gt
            | Token::Lt
            | Token::Ge
            | Token::Le
            | Token::Max
            | Token::Min
            | Token::And
            | Token::Or
            | Token::Not => {
                if !process_stack
                    .last()
                    .ok_or(UnexpectedToken { token, position: i })?
                    .is_empty()
                {
                    return Err(Box::new(UnexpectedToken { token, position: i }));
                }

                process_stack
                    .last_mut()
                    .ok_or(UnexpectedToken { token, position: i })?
                    .push(token);
            }
            Token::Number(_) | Token::Bool(_) => {
                if process_stack
                    .last()
                    .ok_or(UnexpectedToken { token, position: i })?
                    .is_empty()
                {
                    return Err(Box::new(UnexpectedToken { token, position: i }));
                }

                process_stack
                    .last_mut()
                    .ok_or(UnexpectedToken { token, position: i })?
                    .push(token);
            }
            Token::CloseBracket => {
                let pop = process_stack
                    .pop()
                    .ok_or(UnexpectedToken { token, position: i })?;

                let instruction = pop.clone();
                let instruction = instruction.first().unwrap();

                let list = pop
                    .into_iter()
                    .filter_map(|token| match token {
                        Token::Number(num) => Some(num),
                        _ => None,
                    })
                    .collect::<Vec<f64>>();

                if process_stack.is_empty() {
                    process_stack.push(Vec::new());
                }

                match instruction {
                    Token::Add => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter().reduce(|acc, item| acc + item).unwrap(),
                    )),
                    Token::Sub => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter().reduce(|acc, item| acc - item).unwrap(),
                    )),
                    Token::Mul => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter().reduce(|acc, item| acc * item).unwrap(),
                    )),
                    Token::Div => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter().reduce(|acc, item| acc / item).unwrap(),
                    )),
                    Token::Mod => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter().reduce(|acc, item| acc % item).unwrap(),
                    )),
                    Token::Eql => process_stack.last_mut().unwrap().push(Token::Bool(
                        list.first()
                            .map(|first| list.iter().all(|item| item == first))
                            .unwrap_or(true),
                    )),
                    Token::Neql => process_stack.last_mut().unwrap().push(Token::Bool(
                        !list
                            .first()
                            .map(|first| list.iter().all(|item| item == first))
                            .unwrap_or(true),
                    )),
                    Token::Gt => process_stack.last_mut().unwrap().push(Token::Bool(
                        list.windows(2).all(|item| item.first() > item.last()),
                    )),
                    Token::Lt => process_stack.last_mut().unwrap().push(Token::Bool(
                        list.windows(2).all(|item| item.first() < item.last()),
                    )),
                    Token::Ge => process_stack.last_mut().unwrap().push(Token::Bool(
                        list.windows(2).all(|item| item.first() >= item.last()),
                    )),
                    Token::Le => process_stack.last_mut().unwrap().push(Token::Bool(
                        list.windows(2).all(|item| item.first() <= item.last()),
                    )),
                    Token::Max => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter()
                            .reduce(|acc, item| if acc >= item { acc } else { item })
                            .unwrap(),
                    )),
                    Token::Min => process_stack.last_mut().unwrap().push(Token::Number(
                        list.into_iter()
                            .reduce(|acc, item| if acc >= item { item } else { acc })
                            .unwrap(),
                    )),
                    _ => return Err(Box::new(UnexpectedToken { token, position: i })),
                }
            }
        }
    }

    Ok(process_stack)
}
