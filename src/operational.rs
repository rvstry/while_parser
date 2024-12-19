use crate::ast::Stmt;
use crate::denotational::{State, evaluate_arithmetic, evaluate_boolean};

fn execute_statement(statement: &Stmt, state: &mut State) {
    match statement {
        Stmt::Skip => (),
        Stmt::Assn(x, e) => state.update_var(x.clone(), evaluate_arithmetic(e, state)),
        Stmt::Seq(s1, s2) => {execute_statement(s1, state); execute_statement(s2, state);},
        Stmt::Cond(e, s1, s2) => {
            match evaluate_boolean(e, state) {
                true => execute_statement(s1, state),
                false => execute_statement(s2, state),
            };
        }
        Stmt::While(e, s) => {
                match evaluate_boolean(e, state) {
                    false => (),
                    true => {
                        execute_statement(s, state);
                        execute_statement(statement, state);
                }
            }
        }
    };
}
