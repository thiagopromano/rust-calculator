use crate::token::Token::{self, *};

pub fn syntatic_analize(tokens: &Vec<Token>) -> Vec<String> {
    let mut errors = Vec::new();
    let mut parenthesis_count = 0;
    if tokens.len() == 0 {
        errors.push("Error, empty expression is not valid".to_string());
        return errors;
    }
    match tokens[0] {
        Digit(_, _) => (),
        OpenParethesis(_) => {
            parenthesis_count = parenthesis_count + 1;
        }
        _ => {
            errors.push(format!("Error, expression cannot start with {:?}", tokens[0]));
            return errors;
        }
    }

    let mut current_token = tokens[0].clone();

    for t in tokens.iter().skip(1) {
        match t {
            CloseParethesis(pos) => {
                match current_token {
                    Digit(_, _) | CloseParethesis(_) => (),
                    _ => errors.push(format!("Error, unexpected {:?}", t))
                }
                parenthesis_count = parenthesis_count - 1;
                if parenthesis_count < 0 {
                    errors.push(format!("Error, too many closing parenthesis at position {}!" , pos));
                    parenthesis_count = 0;
                }
            }
            OpenParethesis(_) => {
                match current_token {
                    OperationSubtraction(_) | OperationDivision(_) | OperationSum(_) | OperationMultiplication(_) | OpenParethesis(_) => (),
                    _ => errors.push(format!("Error, unexpected {:?}", t))
                }
                parenthesis_count = parenthesis_count + 1;
            }
            Digit(_, _) => {
                match current_token {
                    OperationSubtraction(_) | OperationDivision(_) | OperationSum(_) | OperationMultiplication(_) | OpenParethesis(_) => (),
                    _ => errors.push(format!("Error, unexpected {:?}", t))
                }
            }
            OperationSubtraction(_) | OperationDivision(_) | OperationSum(_) | OperationMultiplication(_) => {
                match current_token {
                    CloseParethesis(_) | Digit(_, _) => (),
                    _ => errors.push(format!("Error, unexpected {:?}", t))
                }
            }
        }
        current_token = t.clone();
    }

    match tokens[tokens.len() - 1] {
        Digit(_, _)  => (),
        CloseParethesis(_) => (),
        _ => {
            errors.push(format!("Error, expression cannot end with {:?}", tokens[tokens.len() - 1]));
        }
    }

    if parenthesis_count != 0 {
        errors.push(format!("Error, missing {} closing parenthesis", parenthesis_count));
    }
    return errors;
}

#[cfg(test)]
mod test_lexic {
    use super::*;

    #[test]
    fn empty_string_gives_nothing() {
        assert_eq!(vec!("Error, empty expression is not valid"), syntatic_analize(&vec!()));
    }

    #[test]
    fn expression_with_errors_should_give_errors() {
        assert_eq!(Vec::<String>::new(), syntatic_analize(&vec!(Digit(0,55))));
    }
}