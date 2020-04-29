use crate::token::Token::{self, *};
use crate::token::OperationType::{Multiplication, Division, Sum, Subtraction, GreaterThan, LowerThan};

pub fn lexic_analize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut digit_array = String::new();
    for (pos, c) in input.chars().enumerate() {
        if c >= '0' && c <= '9' {
            digit_array.push(c);
            continue;
        }
        //ended sequence of numbers
        if digit_array.len() > 0 {
            tokens.push(Digit(pos - digit_array.len(), digit_array.parse().unwrap()));
            digit_array.clear()
        }

        match c {
            ' ' | '\r' | '\n' => (), //ignore whitespace
            '*' => tokens.push(Operation(pos, Multiplication)),
            '/' => tokens.push(Operation(pos, Division)),
            '%' => tokens.push(Operation(pos, Division)),
            '+' => tokens.push(Operation(pos, Sum)),
            '-' => tokens.push(Operation(pos, Subtraction)),
            '>' => tokens.push(Operation(pos, GreaterThan)),
            '<' => tokens.push(Operation(pos, LowerThan)),
            '(' => tokens.push(OpenParethesis(pos)),
            ')' => tokens.push(CloseParethesis(pos)),
            _ => tokens.push(Error(pos, c)),
        }
    }
    if digit_array.len() > 0 {
        tokens.push(Digit(0, digit_array.parse().unwrap()));
        digit_array.clear()
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
            Digit(0, 55),
            Operation(2, Sum),
            Digit(3, 4),
            Operation(4, Division),
            Digit(5, 3),
            Error(6, 'a'),
            Operation(7, Multiplication),
            Error(8, 'a'), Error(9, 'f'),
            Operation(10, Subtraction),
            OpenParethesis(11),
            CloseParethesis(12),
        ), lexic_analize("55+4/3a*af-()"));
    }
}