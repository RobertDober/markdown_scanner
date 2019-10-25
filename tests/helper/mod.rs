extern crate markdown_scanner;

use markdown_scanner as scanner;
use scanner::Token;

pub fn scan(line: &str) -> Vec::<Token> {
    scanner::scan(line.to_string())
}
