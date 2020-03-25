use crate::lexer::Token::Digit;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    Digit(i32),
}

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
            tokens.push(Digit(digit_array.parse().unwrap()));
            digit_array.clear()
        }

        if let Token::Invalid() = token {
            continue;
        }

        match c {
            ' ' | '\r' | '\n' => (), //ignore whitespace
            _ => errors.push(format!("Could not parse token \"{}\" at position {}", c, pos + 1)),
        }

    }
    return (errors, tokens);
}

#[cfg(test)]
mod test_lexic {
    use crate::lexer::lexic_analize;

    #[test]
    fn empty_string_gives_nothing() {
        assert_eq!((vec!(), vec!()), lexic_analize(""));
    }

    #[test]
    fn empty_string_gives_nothing() {
        assert_eq!((vec!(), vec!()), lexic_analize("55 + 4 / 3 aaf + 2"));
    }
}