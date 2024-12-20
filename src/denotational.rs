use std::collections::HashMap;
use crate::ast::{Arithmetic, Boolean};

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

pub fn evaluate_arithmetic(e: &Arithmetic, s: &State) -> i32 {
    match e {
        Arithmetic::Var(x) => s.lookup_var(x.clone()),
        Arithmetic::Num(n) => *n,
        Arithmetic::Plus(e1, e2) => evaluate_arithmetic(e1, s) + evaluate_arithmetic(e2, s),
        Arithmetic::Minus(e1, e2) => evaluate_arithmetic(e1, s) - evaluate_arithmetic(e2, s),
        Arithmetic::Times(e1, e2) => evaluate_arithmetic(e1, s) * evaluate_arithmetic(e2, s),
    }
}

pub fn evaluate_boolean(e: &Boolean, s: &State) -> bool {
    match e {
        Boolean::True => true,
        Boolean::False => false,
        Boolean::Less(e1, e2) => evaluate_arithmetic(e1, s) <= evaluate_arithmetic(e2, s),
        Boolean::Eq(e1, e2) => evaluate_arithmetic(e1, s) == evaluate_arithmetic(e2, s),
        Boolean::Not(e) => !evaluate_boolean(e, s),
        Boolean::And(e1, e2) => evaluate_boolean(e1, s) && evaluate_boolean(e2, s),
        Boolean::Or(e1, e2) => evaluate_boolean(e1, s) || evaluate_boolean(e2, s),
    }
}
