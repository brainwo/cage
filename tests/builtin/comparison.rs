use wage::*;

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
fn not_equal() {
    let code = "(!= 10 42 10 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Bool(true)]]);
    }

    let code = "(!= 10 10 10 10)";

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
fn greater_equal() {
    let code = "(>= 30 30 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Bool(true)]]);
    }

    let code = "(>= 10 42 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_ne!(result.unwrap(), [[Token::Bool(true)]]);
    }
}

#[test]
fn less_equal() {
    let code = "(<= 10 10 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Bool(true)]]);
    }

    let code = "(<= 10 42 30)";

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
        assert_eq!(result.unwrap(), [[Token::Double(30.)]]);
    }
}

#[test]
fn min() {
    let code = "(min 30 20 10 20 30)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[Token::Double(10.)]]);
    }
}
