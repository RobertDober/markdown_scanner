extern crate markdown_scanner;
use markdown_scanner as ms;
use std::io;
use std::io::prelude::*;

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(content) => { println!("{:?}", ms::scan(content)) }
            _           => { println!("{}", "huh") }
        }
    }

}
