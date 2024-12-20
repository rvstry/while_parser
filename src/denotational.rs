use std::collections::HashMap;
use crate::ast::Exp;

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

pub fn evaluate_arithmetic(e: &Exp, s: &State) -> i32 {
    match e {
        Exp::Var(x) => s.lookup_var(x.clone()),
        Exp::Num(n) => *n,
        Exp::Plus(e1, e2) => evaluate_arithmetic(e1, s) + evaluate_arithmetic(e2, s),
        Exp::Minus(e1, e2) => evaluate_arithmetic(e1, s) - evaluate_arithmetic(e2, s),
        Exp::Times(e1, e2) => evaluate_arithmetic(e1, s) * evaluate_arithmetic(e2, s),
        _ => panic!()
    }
}

pub fn evaluate_boolean(e: &Exp, s: &State) -> bool {
    match e {
        Exp::True => true,
        Exp::False => false,
        Exp::Less(e1, e2) => evaluate_arithmetic(e1, s) <= evaluate_arithmetic(e2, s),
        Exp::Eq(e1, e2) => evaluate_arithmetic(e1, s) == evaluate_arithmetic(e2, s),
        Exp::Not(e) => !evaluate_boolean(e, s),
        Exp::And(e1, e2) => evaluate_boolean(e1, s) && evaluate_boolean(e2, s),
        Exp::Or(e1, e2) => evaluate_boolean(e1, s) || evaluate_boolean(e2, s),
        _ => panic!()
    }
}
