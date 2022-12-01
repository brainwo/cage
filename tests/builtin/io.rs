use wage::*;

#[test]

fn print() {
    let code = "(print 10)";

    let result = eval(code);

    assert!(result.is_ok());
    if result.is_ok() {
        assert_eq!(result.unwrap(), [[]]);
    }
}
