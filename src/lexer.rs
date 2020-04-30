use crate::token::Token::{self, *};
use crate::token::OperationType::{*};
use crate::token::Keyword::{*};
use regex::Regex;
use std::panic;

pub fn lexic_analize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut digit_array = String::new();

    let mut curr_pos = 0;
    let end_pos = input.len();
    let re = Regex::new(r"^(?:(?P<number>\d{1,3})|(?P<if>if)|(?P<for>for)|(?P<in>in)|(?P<out>out)|(?P<lt><)|(?P<gt>>)|(?P<not>!)|(?P<eq>=)|(?P<id>\w(?:\d|\w)*))").unwrap();
    loop {
        if curr_pos == end_pos {
            break;
        }
        let curr_str = &input[curr_pos..];
        let captures = match re.captures(curr_str) {
            Some(Captures) => Captures,
            None => {
                curr_pos += 1;
                continue;
            }
        };
        if captures.name("number") != None {
            let capture = captures.name("number").unwrap();
            tokens.push(Token::Number(curr_pos, capture.as_str().parse().unwrap()));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("if") != None {
            let capture = captures.name("if").unwrap();
            tokens.push(Token::Keyword(curr_pos, If));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("for") != None {
            let capture = captures.name("for").unwrap();
            tokens.push(Token::Keyword(curr_pos, For));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("in") != None {
            let capture = captures.name("in").unwrap();
            tokens.push(Token::Keyword(curr_pos, In));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("out") != None {
            let capture = captures.name("out").unwrap();
            tokens.push(Token::Keyword(curr_pos, In));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("lt") != None {
            let capture = captures.name("lt").unwrap();
            tokens.push(Token::Operation(curr_pos, LessThan));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("gt") != None {
            let capture = captures.name("gt").unwrap();
            tokens.push(Token::Operation(curr_pos, GreaterThan));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("not") != None {
            let capture = captures.name("not").unwrap();
            tokens.push(Token::Operation(curr_pos, Not));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("eq") != None {
            let capture = captures.name("eq").unwrap();
            tokens.push(Token::Operation(curr_pos, Equal));
            curr_pos += capture.end();
            continue;
        }
        if captures.name("id") != None {
            let capture = captures.name("id").unwrap();
            tokens.push(Token::ID(curr_pos, capture.as_str().to_string()));
            curr_pos += capture.end();
            continue;
        }
        print!("captures {:?}", captures);
        panic!("unreachable code")
    }
    return tokens;
}

#[cfg(test)]
mod test_lexic {
    use super::*;

    #[test]
    fn empty_string_gives_nothing() {
        assert_eq!(Vec::<Token>::new(), lexic_analize(""));
    }

    #[test]
    fn expression_with_errors_should_give_errors() {
        assert_eq!(vec!(
            Operation(0, LessThan), Keyword(2, If), Keyword(5, For), ID(9, "a".to_string()), Operation(11, Equal), Number(13, 31), Number(18, 252), ID(22, "p3ssego0".to_string()), Operation(30, Not), Operation(31, Equal), Keyword(33, In), Keyword(36, In), ID(40, "amora".to_string()), Operation(45, GreaterThan)
        ), lexic_analize("< if for a = 31 + 252 p3ssego0!= in out amora>"));
    }
}