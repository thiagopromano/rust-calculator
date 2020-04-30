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
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Keyword {
    If,
    For,
    In,
    Out,
}