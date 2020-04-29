use crate::lexer::*;
use crate::sintatical::*;

mod lexer;
mod sintatical;
mod token;

pub struct Config {
    pub input: String,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let input = args[1].clone();

        Ok(Config { input })
    }
}

pub fn run(config: Config) {
    println!("Starting lexical analyzer");
    let tokens = lexic_analize(&config.input[..]);
    let errors = tokens.iter().filter(|tok| matches!(tok, token::Token::Error(_, _)));


    for e in errors {
        println!("{:?}", e)
    }
//    for t in &tokens {
//        println!("Received a {:#?} token", t)
//    }
//    println!("Starting syntatic analyzer");
//    let errors = syntatic_analize(&tokens);
//    for e in &errors {
//        println!("{}", e)
//    }
//    if errors.len() == 0 {
//        println!("The syntatic analizer did not report any errors");
//    }
//    return errors;
}

#[cfg(test)]
mod test_program {
    use super::*;
//
//    #[test]
//    fn empty_string_gives_error() {
//        assert_eq!(vec!("Error, empty expression is not valid"), run(Config { input: "".to_string() }));
//        assert_eq!(vec!("Error, expression cannot start with OperationSum(0)"), run(Config { input: "+3".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error01() {
//        assert_eq!(vec!("Error, expression cannot end with OperationSum(1)"), run(Config { input: "1+a".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error02() {
//        assert_eq!(vec!("Error, expression cannot start with OperationDivision(0)"), run(Config { input: "%+1".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error03() {
//        assert_eq!(vec!("Error, unexpected OperationSubtraction(2)"), run(Config { input: "1+-2".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error04() {
//        assert_eq!(Vec::<String>::new(), run(Config { input: "1+2".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error05() {
//        assert_eq!(vec!("Error, expression cannot start with OperationSubtraction(0)"), run(Config { input: "-1+2".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error06() {
//        assert_eq!(vec!("Error, unexpected CloseParethesis(3)"), run(Config { input: "1+()".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error07() {
//        assert_eq!(Vec::<String>::new(), run(Config { input: "1+(12)".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error08() {
//        assert_eq!(vec!("Error, unexpected OpenParethesis(2)"), run(Config { input: "12(12)".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error09() {
//        assert_eq!(vec!("Error, unexpected CloseParethesis(2)", "Error, too many closing parenthesis at position 2!", "Error, unexpected Digit(3, 12)", "Error, unexpected OpenParethesis(5)", "Error, expression cannot end with OpenParethesis(5)", "Error, missing 1 closing parenthesis"), run(Config { input: "1+)12(".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error11() {
//        assert_eq!(Vec::<String>::new(), run(Config { input: "1+((123+(12+45)/23)+(12+(123/456))+12)+(12-34)/45".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error12() {
//        assert_eq!(Vec::<String>::new(), run(Config { input: "1+(((12+45)/23)+(12+(123/456))+12)+(12-34)/45+(12)".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error13() {
//        assert_eq!(vec!("Error, too many closing parenthesis at position 32!"), run(Config { input: "1+((12+45)/23)+(12+(123/456))+12)+(12-34)/45".to_string() }));
//    }
//
//    #[test]
//    fn empty_string_gives_error14() {
//        assert_eq!(vec!("Error, missing 1 closing parenthesis"), run(Config { input: "1+(((12+45)/23)+(12+(123/456)+12)+(12-34)/45".to_string() }));
//    }
}