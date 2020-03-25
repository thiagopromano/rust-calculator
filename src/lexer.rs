use crate::token::Token::{self, *};

pub fn lexic_analize(input: &str) -> (Vec<String>, Vec<Token>) {
    let mut errors = Vec::new();
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
            '*' => tokens.push(OperationMultiplication(pos)),
            '/' => tokens.push(OperationDivision(pos)),
            '+' => tokens.push(OperationSum(pos)),
            '-' => tokens.push(OperationSubtraction(pos)),
            '(' => tokens.push(OpenParethesis(pos)),
            ')' => tokens.push(CloseParethesis(pos)),
            _ => errors.push(format!("Could not parse token \"{}\" at position {}", c, pos + 1)),
        }
    }
    return (errors, tokens);
}

#[cfg(test)]
mod test_lexic {
    use super::*;

    #[test]
    fn empty_string_gives_nothing() {
        assert_eq!((vec!(), vec!()), lexic_analize(""));
    }

    #[test]
    fn expression_with_errors_should_give_errors() {
        assert_eq!((vec!(
            "Could not parse token \"a\" at position 7".to_string(),
            "Could not parse token \"a\" at position 9".to_string(),
            "Could not parse token \"f\" at position 10".to_string(),
        ), vec!(
            (Digit(0, 55)),
            (OperationSum(2)),
            (Digit(3, 4)),
            (OperationDivision(4)),
            (Digit(5, 3)),
            (OperationMultiplication(7)),
            (OperationSubtraction(10)),
            (OpenParethesis(11)),
            (CloseParethesis(12)),
        )), lexic_analize("55+4/3a*af-()"));
    }
}