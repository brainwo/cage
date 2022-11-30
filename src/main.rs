#[derive(Debug, Clone, Copy)]
enum Token {
    OpenBracket,
    CloseBracket,
    Add,
    Sub,
    Mul,
    Div,
    Number(f64),
}

fn tokenize(code: &str) -> Vec<Token> {
    code.split_inclusive(['(', ')', '+', '-', '*', '/'])
        .map(str::to_string)
        .inspect(|x| println!("{x}"))
        .collect::<Vec<String>>()
        .join(" ")
        .split_whitespace()
        .map(|token| match token {
            "(" => Token::OpenBracket,
            ")" => Token::CloseBracket,
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            token => {
                println!("{token}");
                match token.parse::<f64>() {
                    Ok(number) => Token::Number(number),
                    Err(err) => panic!("(TOKENIZE) Unable to parse: {err}"),
                }
            }
        })
        .collect()
}

fn main() {
    let code = "(+ 1 (* 2 2 5))";

    let stack: Vec<Token> = tokenize(code);

    let mut process_stack: Vec<Vec<Token>> = Vec::new();

    stack.into_iter().for_each(|token| match token {
        Token::OpenBracket => process_stack.push(Vec::new()),
        Token::Add => process_stack.last_mut().unwrap().push(Token::Add),
        Token::Sub => process_stack.last_mut().unwrap().push(Token::Sub),
        Token::Mul => process_stack.last_mut().unwrap().push(Token::Mul),
        Token::Div => process_stack.last_mut().unwrap().push(Token::Div),
        Token::Number(num) => process_stack.last_mut().unwrap().push(Token::Number(num)),
        Token::CloseBracket => {
            let pop = process_stack.pop().unwrap();

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
                    list.into_iter().reduce(|accum, item| accum + item).unwrap(),
                )),
                Token::Sub => process_stack.last_mut().unwrap().push(Token::Number(
                    list.into_iter().reduce(|accum, item| accum - item).unwrap(),
                )),
                Token::Mul => process_stack.last_mut().unwrap().push(Token::Number(
                    list.into_iter().reduce(|accum, item| accum * item).unwrap(),
                )),
                Token::Div => process_stack.last_mut().unwrap().push(Token::Number(
                    list.into_iter().reduce(|accum, item| accum / item).unwrap(),
                )),
                _ => println!("Error"),
            }
        }
    });

    println!("{process_stack:?}");
}
