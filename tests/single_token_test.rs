extern crate markdown_scanner;
mod helper;
use helper::scan;
use markdown_scanner::Token;


#[test]
fn test_blank() {
    let result = scan(" ");
    match result.first() {
        Some(Token::Blank(i)) => assert_eq!(1, *i),
        _                     => assert!(false)
    }
}

#[test]
fn test_empty() {
    let result = scan("");
    match result.first() {
        Some(Token::Blank(i)) => assert_eq!(0, *i),
        _                     => assert!(false)
    }
}
