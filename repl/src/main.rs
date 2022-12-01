use cage::*;
use std::io::Write;

static WELCOME_MESSAGE: &str = "Cage v0.1.0\tCopyright (C) 2022 Brian Wo";

fn main() {
    println!("{WELCOME_MESSAGE}");

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().expect("Error flushing");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match eval(input.trim()) {
            Ok(eval) => eval.iter().for_each(|stack| {
                stack.iter().for_each(|e| match e {
                    Token::Number(n) => println!("\x1B[33m{n}\x1B[0m"),
                    Token::Bool(b) => println!("\x1B[33m{b}\x1B[0m"),
                    _ => println!("{e:?}"),
                })
            }),
            Err(err) => {
                println!("\x1B[31m{}\x1B[0m", input.trim());
                println!("\x1B[31m{}\x1B[0m", err);
            }
        }
    }
}
