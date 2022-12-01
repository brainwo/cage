use wage::*;

#[test]

fn print() {
    let code = "(print 10)";

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
