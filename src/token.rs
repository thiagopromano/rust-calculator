#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Token {
    Digit(usize, i32),
    OperationMultiplication(usize),
    OperationDivision(usize),
    OperationSum(usize),
    OperationSubtraction(usize),
    OpenParethesis(usize),
    CloseParethesis(usize),
}
