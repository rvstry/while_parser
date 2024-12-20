pub trait Ast {
}

#[derive(Debug, Eq, PartialEq)]
pub enum Arithmetic {
    Var(String),
    Num(i32),
    Plus(Box<Arithmetic>, Box<Arithmetic>),
    Minus(Box<Arithmetic>, Box<Arithmetic>),
    Times(Box<Arithmetic>, Box<Arithmetic>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Boolean {
    True,
    False,
    Less(Box<Arithmetic>, Box<Arithmetic>),
    Eq(Box<Arithmetic>, Box<Arithmetic>),
    Not(Box<Boolean>),
    And(Box<Boolean>, Box<Boolean>),
    Or(Box<Boolean>, Box<Boolean>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Exp {
    A(Arithmetic),
    B(Boolean),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Stmt {
    Skip,
    Assn(String, Box<Arithmetic>),
    Seq(Box<Stmt>, Box<Stmt>),
    Cond(Box<Boolean>, Box<Stmt>, Box<Stmt>),
    While(Box<Boolean>, Box<Stmt>),
}
