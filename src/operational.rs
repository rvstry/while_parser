use crate::ast::Stmt;
use crate::denotational::{State, evaluate_arithmetic, evaluate_boolean};
use crate::error::OperationError;

pub fn execute_statement(statement: &Stmt, state: &mut State) -> Result<(), OperationError> {
    match statement {
        Stmt::Skip => Ok(()),
        Stmt::Assn(x, e) => {
            match evaluate_arithmetic(e, state) {
                Ok(a) => {
                    state.update_var(x.clone(), a);
                    Ok(())
                },
                Err(_) => Err(OperationError::Execution),
            }
        },
        Stmt::Seq(s1, s2) => {
            match execute_statement(s1, state) {
                Err(_) => Err(OperationError::Execution),
                Ok(_)=> {
                    match execute_statement(s2, state) {
                        Err(_) => Err(OperationError::Execution),
                        Ok(_)=> Ok(()),
                    }
                },
            }
        }
        Stmt::Cond(e, s1, s2) => {
            match evaluate_boolean(e, state) {
                Ok(true) => match execute_statement(s1, state) {
                    Err(_) => Err(OperationError::Execution),
                    Ok(_) => Ok(()),
                },
                Ok(false) => match execute_statement(s2, state) {
                    Err(_) => Err(OperationError::Execution),
                    Ok(_) => Ok(()),
                },
                Err(_) => Err(OperationError::Execution),
            }
        },
        Stmt::While(e, s) => {
                match evaluate_boolean(e, state) {
                    Err(_) => Err(OperationError::Execution),
                    Ok(false) => Ok(()),
                    Ok(true) => {
                        match execute_statement(s, state) {
                            Ok(_) => {
                                match execute_statement(statement, state) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(OperationError::Execution),
                                }
                            },
                            Err(_) => Err(OperationError::Execution),
                        }
                }
            }
        }
    }
}
