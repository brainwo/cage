use cage::*;
use std::io::BufRead;

static WELCOME_MESSAGE: &str = "Welcome to Not.js v0.1.\nType \".help\" to call a doctor.";

fn main() {
    println!("{WELCOME_MESSAGE}");

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match eval(&line.unwrap()) {
            Ok(eval) => eval.iter().for_each(|stack| {
                stack.iter().for_each(|e| match e {
                    Token::Number(n) => println!("\x1B[33m{n}\x1B[0m"),
                    Token::Bool(b) => println!("\x1B[33m{b}\x1B[0m"),
                    _ => println!("{e:?}"),
                })
            }),
            Err(err) => println!("\x1B[31m{}\x1B[0m", err),
        }
    }
}
