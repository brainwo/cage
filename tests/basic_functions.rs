use cage::*;

#[test]
fn aritmetic_operations() {
    let code = "(+ 10 38 (- 60 (/ 20 (* 100 20 ))))";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(107.99000000000001)]]);
    }
}

#[test]
fn equal() {
    let code = "(= 10 10 10 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Bool(true)]]);
    }

    let code = "(= 10 42 10 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Bool(true)]]);
    }
}

#[test]
fn less_than() {
    let code = "(< 10 20 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Bool(true)]]);
    }

    let code = "(< 10 42 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Bool(true)]]);
    }
}

#[test]
fn greater_than() {
    let code = "(> 30 20 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Bool(true)]]);
    }

    let code = "(> 10 42 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Bool(true)]]);
    }
}

#[test]
fn max() {
    let code = "(max 10 20 30 20 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(30.)]]);
    }
}

#[test]
fn min() {
    let code = "(min 30 20 10 20 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Number(10.)]]);
    }
}
