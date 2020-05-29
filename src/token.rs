#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
    Number(usize, i32),
    Operation(usize, OperationType),
    OpenParethesis(usize),
    CloseParethesis(usize),
    Keyword(usize, Keyword),
    Error(usize, char),
    ID(usize, String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Number(_, number) => format!("{}", number),
            Token::Operation(_, op) => op.to_string(),
            Token::OpenParethesis(_) => String::from("("),
            Token::CloseParethesis(_) => String::from(")"),
            Token::Keyword(_, op) => op.to_string(),
            Token::Error(_, op) => op.to_string(),
            Token::ID(_, op) => op.to_string(),
            _ => panic!("unhandled type")
        }
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum OperationType {
    Sum,
    Multiplication,
    Subtraction,
    Division,
    GreaterThan,
    Not,
    Equal,
    LessThan,
}

impl OperationType {
    pub fn to_string(&self) -> String {
        match self {
            OperationType::Sum => String::from("+"),
            OperationType::Multiplication => String::from("*"),
            OperationType::Subtraction => String::from("-"),
            OperationType::Division => String::from("/"),
            OperationType::GreaterThan => String::from(">"),
            OperationType::Not => String::from("!"),
            OperationType::Equal => String::from("="),
            OperationType::LessThan => String::from("<"),
        }
    }
}


#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Keyword {
    If,
    For,
    In,
    Out,
}

impl Keyword {
    pub fn to_string(&self) -> String {
        match self {
            Keyword::If => String::from("if"),
            Keyword::For => String::from("for"),
            Keyword::In => String::from("in"),
            Keyword::Out => String::from("out"),
        }
    }
}