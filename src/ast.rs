pub trait Ast {
}

#[derive(Debug, Eq, PartialEq)]
pub enum Exp {
    True,
    False,
    Less(Box<Exp>, Box<Exp>),
    Eq(Box<Exp>, Box<Exp>),
    Not(Box<Exp>),
    And(Box<Exp>, Box<Exp>),
    Or(Box<Exp>, Box<Exp>),
    Var(String),
    Num(i32),
    Plus(Box<Exp>, Box<Exp>),
    Minus(Box<Exp>, Box<Exp>),
    Times(Box<Exp>, Box<Exp>),
}

impl Ast for Exp {}

#[derive(Debug, Eq, PartialEq)]
pub enum Stmt {
    Skip,
    Assn(String, Box<Exp>),
    Seq(Box<Stmt>, Box<Stmt>),
    Cond(Box<Exp>, Box<Stmt>, Box<Stmt>),
    While(Box<Exp>, Box<Stmt>),
}

impl Ast for Stmt {}
