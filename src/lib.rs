mod error;

use error::*;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    Float(f32),
    Double(f64),
    Bool(bool),
    String(String),
    // Input output
    Print,
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
                "print" => Token::Print,
                token => {
                    if token.starts_with('\"') && token.ends_with('\"') {
                        return Ok(Token::String(token.replace('\"', "")));
                    }
                    let parse = token.parse::<f64>()?;
                    if parse >= f32::MIN.into() && parse <= f32::MAX.into() {
                        Token::Float(parse as f32)
                    } else {
                        Token::Double(parse)
                    }
                }
            })
        })
        .collect()
}

pub fn eval(code: &str) -> Result<Vec<Vec<Token>>, Box<dyn Error>> {
    let stack: Vec<Token> = tokenize(code)?;

    // println!("{:?}", stack);

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
            | Token::Not
            | Token::Print => {
                if !process_stack
                    .last()
                    .ok_or(UnexpectedToken {
                        token: token.clone(),
                        position: i,
                    })?
                    .is_empty()
                {
                    return Err(Box::new(UnexpectedToken { token, position: i }));
                }

                process_stack
                    .last_mut()
                    .ok_or(UnexpectedToken {
                        token: token.clone(),
                        position: i,
                    })?
                    .push(token);
            }
            Token::Float(_) | Token::Double(_) | Token::Bool(_) | Token::String(_) => {
                if process_stack
                    .last()
                    .ok_or(UnexpectedToken {
                        token: token.clone(),
                        position: i,
                    })?
                    .is_empty()
                {
                    return Err(Box::new(UnexpectedToken { token, position: i }));
                }

                process_stack
                    .last_mut()
                    .ok_or(UnexpectedToken {
                        token: token.clone(),
                        position: i,
                    })?
                    .push(token);
            }
            Token::CloseBracket => {
                let pop = process_stack.pop().ok_or(UnexpectedToken {
                    token: token.clone(),
                    position: i,
                })?;

                let instruction = pop.clone();
                let instruction = instruction.first().unwrap();

                if pop.iter().any(|token| matches!(token, Token::String(_))) {
                    let list = pop
                        .into_iter()
                        .filter_map(|token| match token {
                            Token::Double(num) => Some(num.to_string()),
                            Token::Bool(bool) => Some(bool.to_string()),
                            Token::String(string) => Some(string),
                            _ => None,
                        })
                        .collect::<Vec<String>>();

                    println!("{}", list.join(" "));

                    break;
                }

                let list = pop
                    .into_iter()
                    .filter_map(|token| match token {
                        Token::Float(num) => Some(num.into()),
                        Token::Double(num) => Some(num),
                        _ => None,
                    })
                    .collect::<Vec<f64>>();

                if process_stack.is_empty() {
                    process_stack.push(Vec::new());
                }

                match instruction {
                    Token::Add => process_stack.last_mut().unwrap().push(Token::Double(
                        list.into_iter().reduce(|acc, item| acc + item).unwrap(),
                    )),
                    Token::Sub => process_stack.last_mut().unwrap().push(Token::Double(
                        list.into_iter().reduce(|acc, item| acc - item).unwrap(),
                    )),
                    Token::Mul => process_stack.last_mut().unwrap().push(Token::Double(
                        list.into_iter().reduce(|acc, item| acc * item).unwrap(),
                    )),
                    Token::Div => process_stack.last_mut().unwrap().push(Token::Double(
                        list.into_iter().reduce(|acc, item| acc / item).unwrap(),
                    )),
                    Token::Mod => process_stack.last_mut().unwrap().push(Token::Double(
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
                    Token::Max => process_stack.last_mut().unwrap().push(Token::Double(
                        list.into_iter()
                            .reduce(|acc, item| if acc >= item { acc } else { item })
                            .unwrap(),
                    )),
                    Token::Min => process_stack.last_mut().unwrap().push(Token::Double(
                        list.into_iter()
                            .reduce(|acc, item| if acc >= item { item } else { acc })
                            .unwrap(),
                    )),
                    Token::Print => {
                        println!(
                            "{}",
                            list.into_iter()
                                .map(|item| item.to_string())
                                .collect::<Vec<String>>()
                                .join(" ")
                        )
                    }
                    _ => return Err(Box::new(UnexpectedToken { token, position: i })),
                }
            }
        }
    }

    Ok(process_stack)
}
