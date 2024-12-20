use std::collections::VecDeque;
use crate::token::Token;
use crate::ast::*;
use crate::error::ParseError;

struct Parser {
    input: VecDeque<Token>,
}

impl Parser {

    fn init(s: VecDeque<Token>) -> Self {
        Self {input: s,}
    }


    fn peek(&self) -> Result<Token, ParseError> /*Option<Token>*/ {
        self.input.front().cloned().ok_or(ParseError::Token)

    }

    fn eat(&mut self, tk: Token) -> Result<(), ParseError> {
        if tk == self.peek()? {
            // consume
            let mut p = self.input.clone();
            self.input = p.split_off(1);
            Ok(())
        }
        else {Err(ParseError::Token)}

    }

    fn parse_prog(&mut self) -> Result<Stmt, ParseError> {
        let c = self.peek()?;
        match c {
            Token::If => {
                let st1 = self.parse_stmt()?; let st2 = self.parse_stmts(st1); self.eat(Token::Dollar)?;
                st2
            },
            Token::While => {
                let st1 = self.parse_stmt()?; let st2 = self.parse_stmts(st1); self.eat(Token::Dollar)?;
                st2
            },
            Token::Skip => {
                let st1 = self.parse_stmt()?; let st2 = self.parse_stmts(st1); self.eat(Token::Dollar)?;
                st2
           },
            Token::Id(_) => {
                let st1 = self.parse_stmt()?; let st2 = self.parse_stmts(st1); self.eat(Token::Dollar)?;
                st2
         },
            Token::LeftCurly => {
                let st1 = self.parse_stmt()?; let st2 = self.parse_stmts(st1); self.eat(Token::Dollar)?;
                st2
             },
            _ => Err(ParseError::Prog),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let c = self.peek()?;
        match c {
            Token::If => {
                self.eat(Token::If)?; let s1 = self.parse_bexp()?; self.eat(Token::Then)?; let s2 = self.parse_stmt()?; self.eat(Token::Else)?; let s3 = self.parse_stmt()?;
                Ok(Stmt::Cond(Box::new(s1), Box::new(s2), Box::new(s3)))
            },
            Token::While => {
                self.eat(Token::While)?; let s1 = self.parse_bexp()?; self.eat(Token::Do)?; let s2 = self.parse_stmt()?;
                Ok(Stmt::While(Box::new(s1), Box::new(s2)))
            },
            Token::Skip => {
                self.eat(Token::Skip)?;
                Ok(Stmt::Skip)
            },
            Token::Id(s) => {
                self.eat(Token::Id(s.clone()))?; self.eat(Token::Assignment)?; let s1 = self.parse_aexp()?;
                Ok(Stmt::Assn(s, Box::new(s1)))
            },
            Token::LeftCurly => {
                self.eat(Token::LeftCurly)?; let s1 = self.parse_stmt()?; let s2 = self.parse_stmts(s1); self.eat(Token::RightCurly)?;
                s2
            },
            _ => Err(ParseError::Stmt),

        }
    }


    fn parse_stmts(&mut self, sub: Stmt/*impl Ast*/) -> Result<Stmt, ParseError>  {
        let c = self.peek()?;
        match c {
            Token::Dollar => {/**/Ok(sub)},
            Token::RightCurly => {/**/Ok(sub)},
            Token::Semicolon => {
                self.eat(Token::Semicolon)?; let s1 = self.parse_stmt()?; let s2 = self.parse_stmts(s1)?;
                Ok(Stmt::Seq(Box::new(sub), Box::new(s2)))
            },
            _ => Err(ParseError::Stmts),

        }
    }


    fn parse_bexp(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(_) => {
                let st1 = self.parse_bfac()?;
                self.parse_bexps(st1)
            },
            Token::Not => {
                let st1 = self.parse_bfac()?;
                self.parse_bexps(st1)
            },
            Token::Num(_) => {
                let st1 = self.parse_bfac()?;
                self.parse_bexps(st1)
            },
            Token::True => {
                let st1 = self.parse_bfac()?;
                self.parse_bexps(st1)
            },
            Token::False => {
                let st1 = self.parse_bfac()?;
                self.parse_bexps(st1)
            },
            Token::LeftParenthesis => {
                let st1 = self.parse_bfac()?;
                self.parse_bexps(st1)
            },
            _ => Err(ParseError::BExp),

        }
    }

    fn parse_bexps(&mut self, sub: Exp /*impl Ast*/) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Then => {/**/Ok(sub)},
            Token::Do => {/**/Ok(sub)},
            Token::Or => {
                self.eat(Token::Or)?; let s1 = self.parse_bfac()?; let s2 = self.parse_bexps(s1)?;
                Ok(Exp::Or(Box::new(sub), Box::new(s2)))
            },
            Token::RightParenthesis => {/**/Ok(sub)},
            _ => Err(ParseError::BExps),

        }
    }


    fn parse_bfac(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(_) => {
                let st1 = self.parse_bneg()?;
                self.parse_bfacs(st1)
            },
            Token::Not => {
                let st1 = self.parse_bneg()?;
                self.parse_bfacs(st1)
             },
            Token::Num(_) => {
                let st1 = self.parse_bneg()?;
                self.parse_bfacs(st1)
             },
            Token::True => {
                let st1 = self.parse_bneg()?;
                self.parse_bfacs(st1)
             },
            Token::False => {
                let st1 = self.parse_bneg()?;
                self.parse_bfacs(st1)
             },
            Token::LeftParenthesis => {
                let st1 = self.parse_bneg()?;
                self.parse_bfacs(st1)
             },
            _ => Err(ParseError::BFac),

        }
    }

    fn parse_bfacs(&mut self, sub: Exp/*impl Ast*/) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Then => {/**/Ok(sub)},
            Token::Do => {/**/Ok(sub)},
            Token::Or => {/**/Ok(sub)},
            Token::And => {
                self.eat(Token::And)?; let s1 = self.parse_bneg()?; let s2 = self.parse_bfacs(s1)?;
                Ok(Exp::And(Box::new(sub), Box::new(s2)))
            },
            Token::RightParenthesis => {/**/Ok(sub)},
            _ => Err(ParseError::BFacs),

        }
    }

    fn parse_bneg(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(_) => {
                self.parse_brel()
            },
            Token::Not => {
                self.eat(Token::Not)?; let s1 = self.parse_bneg()?;
                Ok(Exp::Not(Box::new(s1)))
            },
            Token::Num(_) => {
                self.parse_brel()
            },
            Token::True => {
                self.parse_brel()
            },
            Token::False => {
                self.parse_brel()
            },
            Token::LeftParenthesis => {
                self.parse_brel()
            },
            _ => Err(ParseError::BNeg),

        }
    }


    fn parse_brel(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(_) => {
                let st1 = self.parse_aexp()?;
                self.parse_brels(st1)
            },
            Token::Num(_) => {
                let st1 = self.parse_aexp()?;
                self.parse_brels(st1)
            },
            Token::True => {
                let st1 = self.parse_aexp()?;
                self.parse_brels(st1)
            },
            Token::False => {
                let st1 = self.parse_aexp()?;
                self.parse_brels(st1)
            },
            Token::LeftParenthesis => {
                let st1 = self.parse_aexp()?;
                self.parse_brels(st1)
            },
            _ => Err(ParseError::BRel),

        }
    }


    fn parse_brels(&mut self, sub: Exp/*impl Ast*/) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Then => {/**/Ok(sub)},
            Token::Do => {/**/Ok(sub)},
            Token::Or => {/**/Ok(sub)},
            Token::And => {/**/Ok(sub)},
            Token::LessThan => {
                self.eat(Token::LessThan)?; let s1 = self.parse_aexp()?;
                Ok(Exp::Less(Box::new(sub), Box::new(s1)))
            },
            Token::Equals => {
                self.eat(Token::Equals)?; let s1 = self.parse_aexp()?;
                Ok(Exp::Eq(Box::new(sub), Box::new(s1)))
            },
            Token::RightParenthesis => {/**/Ok(sub)},
            _ => Err(ParseError::BRels),

        }
    }


    fn parse_aexp(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(_) => {
                let st1 = self.parse_afac()?;
                self.parse_aexps(st1)
            },
            Token::Num(_) => {
                let st1 = self.parse_afac()?;
                self.parse_aexps(st1)
            },
            Token::True => {
                let st1 = self.parse_afac()?;
                self.parse_aexps(st1)
            },
            Token::False => {
                let st1 = self.parse_afac()?;
                self.parse_aexps(st1)
            },
            Token::LeftParenthesis => {
                let st1 = self.parse_afac()?;
                self.parse_aexps(st1)
            },
            _ => Err(ParseError::AExp),

        }
    }


    fn parse_aexps(&mut self, sub: Exp/*impl Ast*/) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Dollar => {/**/Ok(sub)},
            Token::Then =>{/**/Ok(sub)},
            Token::Else => {/**/Ok(sub)},
            Token::Do => {/**/Ok(sub)},
            Token::RightCurly => {/**/Ok(sub)},
            Token::Semicolon => {/**/Ok(sub)},
            Token::Or => {/**/Ok(sub)},
            Token::And => {/**/Ok(sub)},
            Token::LessThan => {/**/Ok(sub)},
            Token::Equals => {/**/Ok(sub)},
            Token::Plus => {
                self.eat(Token::Plus)?; let s1 = self.parse_afac()?; let s2 = self.parse_aexps(Exp::Plus(Box::new(sub), Box::new(s1)))?;
                Ok(s2)
            },
            Token::Minus => {
                self.eat(Token::Minus)?; let s1 = self.parse_afac()?; let s2 = self.parse_aexps(Exp::Minus(Box::new(sub), Box::new(s1)))?;
                Ok(s2)
            },
            Token::RightParenthesis => {/**/Ok(sub)},
            _ => Err(ParseError::AExps),

        }
    }

    fn parse_afac(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(_) => {
                let st1 = self.parse_atom()?;
                self.parse_afacs(st1)
            },
            Token::Num(_) => {
                let st1 = self.parse_atom()?;
                self.parse_afacs(st1)
             },
            Token::True => {
                let st1 = self.parse_atom()?;
                self.parse_afacs(st1)
             },
            Token::False => {
                let st1 = self.parse_atom()?;
                self.parse_afacs(st1)
             },
            Token::LeftParenthesis => {
                let st1 = self.parse_atom()?;
                self.parse_afacs(st1)
             },
            _ => Err(ParseError::AFac),

        }
    }

    fn parse_afacs(&mut self, sub: Exp/*impl Ast*/) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Dollar => {/**/Ok(sub)},
            Token::Then => {/**/Ok(sub)},
            Token::Else => {/**/Ok(sub)},
            Token::Do => {/**/Ok(sub)},
            Token::RightCurly => {/**/Ok(sub)},
            Token::Semicolon => {/**/Ok(sub)},
            Token::Or => {/**/Ok(sub)},
            Token::And => {/**/Ok(sub)},
            Token::LessThan =>{/**/Ok(sub)},
            Token::Equals =>{/**/Ok(sub)},
            Token::Plus => {/**/Ok(sub)},
            Token::Minus => {/**/Ok(sub)},
            Token::Asterisk => {
                self.eat(Token::Asterisk)?; let s1 = self.parse_atom()?; let s2 = self.parse_afacs(s1)?;
                Ok(Exp::Times(Box::new(sub), Box::new(s2)))
            },
            Token::RightParenthesis => {/**/Ok(sub)},
            _ => Err(ParseError::AFacs),

        }
    }
    fn parse_atom(&mut self) -> Result<Exp, ParseError> {
        let c = self.peek()?;
        match c {
            Token::Id(s) => {
                self.eat(Token::Id(s.clone()))?;
                Ok(Exp::Var(s.to_string()))
            },
            Token::Num(s) => {
                self.eat(Token::Num(s.clone()))?;
                Ok(Exp::Num(s.parse().unwrap()))
            },
            Token::True => {
                self.eat(Token::True)?;
                Ok(Exp::True)
            },
            Token::False => {
                self.eat(Token::False)?;
                Ok(Exp::False)
            },
            Token::LeftParenthesis => {
                self.eat(Token::LeftParenthesis)?; let st = self.parse_bexp()?; self.eat(Token::RightParenthesis)?;
                Ok(st)
            },
            _ => Err(ParseError::Atom),

        }
}
}

pub fn parse(input: &VecDeque<Token>) -> Result<Stmt, ParseError> {
    let mut parser = Parser::init(input.clone());
     parser.parse_prog() // all programs must start with prog
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test00() {
        assert_eq!(parse(&VecDeque::from([Id(String::from("myVar")), Assignment, Id(String::from("x")), Asterisk, LeftParenthesis, Id(String::from("foo")), Plus, Id(String::from("bar")), RightParenthesis, Dollar])), Ok(Stmt::Assn(String::from("myVar"), Box::new(Exp::Times(Box::new(Exp::Var(String::from("x"))), Box::new(Exp::Plus(Box::new(Exp::Var(String::from("foo"))), Box::new(Exp::Var(String::from("bar"))))))))))
    }

    #[test]
    fn test11() {
        assert_eq!(parse(&VecDeque::from([While, True, Do, Skip, Dollar])), Ok(Stmt::While(Box::new(Exp::True), Box::new(Stmt::Skip))))
    }

    #[test]
    fn test22() {
        assert_eq!(parse(&VecDeque::from([If, Id(String::from("x")), LessThan, Equals, Num(String::from("3")), Then, Id(String::from("x")), Assignment, Id(String::from("x")), Minus, Num(String::from("1")), Else, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Dollar])), Err(ParseError::AExp))
    }

    #[test]
    fn test33() {
        assert_eq!(parse(&VecDeque::from([While, Id(String::from("y")), Plus, Num(String::from("3")), LessThan, Num(String::from("2")), Do, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Dollar])), Ok(Stmt::Seq(Box::new(Stmt::While(Box::new(Exp::Less(Box::new(Exp::Plus(Box::new(Exp::Var(String::from("y"))), Box::new(Exp::Num(3)))), Box::new(Exp::Num(2)))), Box::new(Stmt::Assn(String::from("y"), Box::new(Exp::Plus(Box::new(Exp::Var(String::from("y"))), Box::new(Exp::Num(1)))))))), Box::new(Stmt::Assn(String::from("x"), Box::new(Exp::Num(0)))))))
    }

    #[test]
    fn test44() {
        assert_eq!(parse(&VecDeque::from([Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Semicolon, Dollar])), Err(ParseError::Stmt))
    }

    #[test]
    fn test55() {
        assert_eq!(parse(&VecDeque::from([Id(String::from("whiley")), Assignment, LeftParenthesis, Id(String::from("iff")), Plus, Id(String::from("sskip")), RightParenthesis, Asterisk, Id(String::from("doo")), Minus, Id(String::from("thenn")), Dollar])), Ok(Stmt::Assn(String::from("whiley"), Box::new(Exp::Minus(Box::new(Exp::Times(Box::new(Exp::Plus(Box::new(Exp::Var(String::from("iff"))), Box::new(Exp::Var(String::from("sskip"))))),Box::new(Exp::Var(String::from("doo"))))), Box::new(Exp::Var(String::from("thenn"))))))))
    }

    #[test]
    fn test66() {
        assert_eq!(parse(&VecDeque::from([While, Id(String::from("a")), And, Id(String::from("b")), And, Id(String::from("c")), Do, Skip, Dollar])), Ok(Stmt::While(Box::new(Exp::And(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::And(Box::new(Exp::Var(String::from("b"))), Box::new(Exp::Var(String::from("c"))))))), Box::new(Stmt::Skip))))
    }

    #[test]
    fn test77() {
        assert_eq!(parse(&VecDeque::from([While, Id(String::from("a")), Or, Id(String::from("b")), Or, Id(String::from("c")), Do, Skip, Dollar])), Ok(Stmt::While(Box::new(Exp::Or(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::Or(Box::new(Exp::Var(String::from("b"))), Box::new(Exp::Var(String::from("c"))))))), Box::new(Stmt::Skip))))
    }

    #[test]
    fn test88() {
        assert_eq!(parse(&VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Plus, Id(String::from("b")), Plus, Id(String::from("c")), Dollar])), Ok(Stmt::Assn(String::from("x"), Box::new(Exp::Plus(Box::new(Exp::Plus(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::Var(String::from("b"))))),Box::new(Exp::Var(String::from("c"))))))))
    }

    #[test]
    fn test99() {
        assert_eq!(parse(&VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Minus, Id(String::from("b")), Minus, Id(String::from("c")), Dollar])), Ok(Stmt::Assn(String::from("x"), Box::new(Exp::Minus(Box::new(Exp::Minus(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::Var(String::from("b"))))),Box::new(Exp::Var(String::from("c"))))))))
    }
}
