#[derive(Debug, Eq, PartialEq)]
pub enum LexError {
    WhileError,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Token,
    Prog,
    Stmt,
    Stmts,
    BExp,
    BExps,
    BFac,
    BFacs,
    BNeg,
    BRel,
    BRels,
    AExp,
    AExps,
    AFac,
    AFacs,
    Atom,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DenotationError {
    Arithmetic,
    Boolean,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OperationError {
    Execution,
}
