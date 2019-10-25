#![allow(dead_code)]
use std::vec::Vec;
mod version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        assert_eq!(version(), "0.0.1")
    }
}
pub fn version() -> String {
    version::vsn()
}

#[derive(Debug)]
pub enum Token {
    Blank(i64),
    Verb(String),
}

#[derive(Debug)]
enum State {
    Start,
    Indent,
    Verb
}

impl State {
    fn indent_trigger(ch: char, result: &mut Vec<Token>) -> State {
        match ch {
            ' ' => { 
                if let Some(Token::Blank(i)) = result.last_mut() {
                    *i += 1
                };
                State::Indent
            },
            any =>  {
                result.push(Token::Verb(any.to_string()));
                State::Verb
            }
        }
    }
    fn trigger(state: State, ch: char, result: &mut Vec<Token>) -> State {
        match state {
            State::Start => { State::start_trigger(ch, result) },
            State::Indent => { State::indent_trigger(ch, result) },
            State::Verb   => { State::verb_trigger(ch, result) },
        }
    }
    fn start_trigger(ch: char, result: &mut Vec<Token>) -> State {
        match ch {
            ' ' => {
                result.push(Token::Blank(1));
                State::Indent
            },
            any => {
                result.push(Token::Verb(any.to_string()));
                State::Verb
            }
        }
    }

    fn verb_trigger(ch: char, result: &mut Vec<Token>) -> State {
        if let Some(Token::Verb(verb)) = result.last_mut() {
            verb.push(ch);
        }
        State::Verb
    }

    fn finish_default(_result: &mut Vec<Token>) {
    }

    fn finish_start(result: &mut Vec<Token>) {
        result.push(Token::Blank(0));
    }

    fn finish_indent(result: &mut Vec<Token>) {
        if result.len() == 0 {
            result.push(Token::Blank(0));
        }
    }

    fn finish(state: State, result: &mut Vec<Token>) {
        match state {
            State::Start => State::finish_start(result),
            State::Indent => State::finish_indent(result),
            _             => State::finish_default(result)
        }
    }
}

pub fn scan(line: String ) -> Vec<Token>  {
    let mut result = Vec::<Token>::new();
    let mut state  = State::Start;

    for ch in line.chars() {
        state = State::trigger(state, ch, &mut result)
    };
    State::finish(state, &mut result);

    result

}
