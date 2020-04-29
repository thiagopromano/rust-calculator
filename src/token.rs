#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Token {
    Digit(usize, i32),
    Operation(usize, OperationType),
    OpenParethesis(usize),
    CloseParethesis(usize),
    Error(usize, char)
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
    LowerThan,
}