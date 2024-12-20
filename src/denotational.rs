use std::collections::HashMap;
use crate::ast::Exp;
use crate::error::DenotationError;
use std::fmt;

#[derive(Eq,Hash, PartialEq)]
enum Variable {
    X,
    Y,
}

#[derive(Debug)]
pub struct State {
    state: HashMap<String, i32>
}

impl State {
    pub fn new() -> Self {
        Self{
            state: HashMap::new()
        }
    }

    pub fn lookup_var(&self, var: String) -> i32 {
        let lookup = self.state.get(&var).cloned();
        match lookup {
            Some(value) => value,
            None => 0,
        }
    }

    pub fn update_var(&mut self, var: String, value: i32) {
        self.state.insert(var, value);
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mappings: Vec<String> = self.state.iter().map(|(var, val)| {format!("{} \u{21A6} {}", var, val)}).collect();
        write!(f, "[{}]", mappings.join(","))
    }
}

pub fn evaluate_arithmetic(e: &Exp, s: &State) -> Result<i32, DenotationError>  {
    match e {
        Exp::Var(x) => Ok(s.lookup_var(x.clone())),
        Exp::Num(n) => Ok(*n),
        Exp::Plus(e1, e2) => Ok(evaluate_arithmetic(e1, s)? + evaluate_arithmetic(e2, s)?),
        Exp::Minus(e1, e2) => Ok(evaluate_arithmetic(e1, s)? - evaluate_arithmetic(e2, s)?),
        Exp::Times(e1, e2) => Ok(evaluate_arithmetic(e1, s)? * evaluate_arithmetic(e2, s)?),
        _ => Err(DenotationError::Arithmetic),
    }
}

pub fn evaluate_boolean(e: &Exp, s: &State) -> Result<bool, DenotationError> {
    match e {
        Exp::True => Ok(true),
        Exp::False => Ok(false),
        Exp::Less(e1, e2) => Ok(evaluate_arithmetic(e1, s)? <= evaluate_arithmetic(e2, s)?),
        Exp::Eq(e1, e2) => Ok(evaluate_arithmetic(e1, s)? == evaluate_arithmetic(e2, s)?),
        Exp::Not(e) => Ok(!evaluate_boolean(e, s)?),
        Exp::And(e1, e2) => Ok(evaluate_boolean(e1, s)? && evaluate_boolean(e2, s)?),
        Exp::Or(e1, e2) => Ok(evaluate_boolean(e1, s)? || evaluate_boolean(e2, s)?),
        _ => Err(DenotationError::Boolean),
    }
}
