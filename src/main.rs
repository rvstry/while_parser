mod ast;
mod error;
mod lexer;
mod parser;
mod token;

mod denotational;
mod operational;

fn main() {

}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use token::Token::*;
    use ast::Stmt;
    use ast::Exp;
    use error::ParseError;

    #[test]
    fn test0() {
        assert_eq!(lexer::lex("myVar <- x * (foo + bar)"), Ok(VecDeque::from([Id(String::from("myVar")), Assignment, Id(String::from("x")), Asterisk, LeftParenthesis, Id(String::from("foo")), Plus, Id(String::from("bar")), RightParenthesis, Dollar])))
    }

    #[test]
    fn test1() {
        assert_eq!(lexer::lex("while true do skip"), Ok(VecDeque::from([While, True, Do, Skip, Dollar])))
    }

    #[test]
    fn test2() {
        assert_eq!(lexer::lex("if x <= 3 then x <- x - 1 else y <- y + 1"), Ok(VecDeque::from([If, Id(String::from("x")), LessThan, Equals, Num(String::from("3")), Then, Id(String::from("x")), Assignment, Id(String::from("x")), Minus, Num(String::from("1")), Else, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Dollar])))
    }

    #[test]
    fn test3() {
        assert_eq!(lexer::lex("while y + 3 < 2 do y <- y + 1; x <- 0"), Ok(VecDeque::from([While, Id(String::from("y")), Plus, Num(String::from("3")), LessThan, Num(String::from("2")), Do, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Dollar])))
    }

    #[test]
    fn test4() {
        assert_eq!(lexer::lex("y <- y + 1; x <- 0;"), Ok(VecDeque::from([Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Semicolon, Dollar])))
    }

    #[test]
    fn test5() {
        assert_eq!(lexer::lex("whiley <- (iff + sskip) * doo - thenn"), Ok(VecDeque::from([Id(String::from("whiley")), Assignment, LeftParenthesis, Id(String::from("iff")), Plus, Id(String::from("sskip")), RightParenthesis, Asterisk, Id(String::from("doo")), Minus, Id(String::from("thenn")), Dollar])))
    }

    #[test]
    fn test6() {
        assert_eq!(lexer::lex("while a && b && c do skip"), Ok(VecDeque::from([While, Id(String::from("a")), And, Id(String::from("b")), And, Id(String::from("c")), Do, Skip, Dollar])))
    }

    #[test]
    fn test7() {
        assert_eq!(lexer::lex("while a || b || c do skip"), Ok(VecDeque::from([While, Id(String::from("a")), Or, Id(String::from("b")), Or, Id(String::from("c")), Do, Skip, Dollar])))
    }

    #[test]
    fn test8() {
        assert_eq!(lexer::lex("x <- a + b + c"), Ok(VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Plus, Id(String::from("b")), Plus, Id(String::from("c")), Dollar])))
    }

    #[test]
    fn test9() {
        assert_eq!(lexer::lex("x <- a - b - c"), Ok(VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Minus, Id(String::from("b")), Minus, Id(String::from("c")), Dollar])))
    }

    #[test]
    fn test00() {
        assert_eq!(parser::parse(&VecDeque::from([Id(String::from("myVar")), Assignment, Id(String::from("x")), Asterisk, LeftParenthesis, Id(String::from("foo")), Plus, Id(String::from("bar")), RightParenthesis, Dollar])), Ok(Stmt::Assn(String::from("myVar"), Box::new(Exp::Times(Box::new(Exp::Var(String::from("x"))), Box::new(Exp::Plus(Box::new(Exp::Var(String::from("foo"))), Box::new(Exp::Var(String::from("bar"))))))))))
    }

    #[test]
    fn test11() {
        assert_eq!(parser::parse(&VecDeque::from([While, True, Do, Skip, Dollar])), Ok(Stmt::While(Box::new(Exp::True), Box::new(Stmt::Skip))))
    }

    #[test]
    fn test22() {
        assert_eq!(parser::parse(&VecDeque::from([If, Id(String::from("x")), LessThan, Equals, Num(String::from("3")), Then, Id(String::from("x")), Assignment, Id(String::from("x")), Minus, Num(String::from("1")), Else, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Dollar])), Err(ParseError::ParseAExpError))
    }

    #[test]
    fn test33() {
        assert_eq!(parser::parse(&VecDeque::from([While, Id(String::from("y")), Plus, Num(String::from("3")), LessThan, Num(String::from("2")), Do, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Dollar])), Ok(Stmt::Seq(Box::new(Stmt::While(Box::new(Exp::Less(Box::new(Exp::Plus(Box::new(Exp::Var(String::from("y"))), Box::new(Exp::Num(3)))), Box::new(Exp::Num(2)))), Box::new(Stmt::Assn(String::from("y"), Box::new(Exp::Plus(Box::new(Exp::Var(String::from("y"))), Box::new(Exp::Num(1)))))))), Box::new(Stmt::Assn(String::from("x"), Box::new(Exp::Num(0)))))))
    }

    #[test]
    fn test44() {
        assert_eq!(parser::parse(&VecDeque::from([Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Semicolon, Dollar])), Err(ParseError::ParseStmtError))
    }

    #[test]
    fn test55() {
        assert_eq!(parser::parse(&VecDeque::from([Id(String::from("whiley")), Assignment, LeftParenthesis, Id(String::from("iff")), Plus, Id(String::from("sskip")), RightParenthesis, Asterisk, Id(String::from("doo")), Minus, Id(String::from("thenn")), Dollar])), Ok(Stmt::Assn(String::from("whiley"), Box::new(Exp::Minus(Box::new(Exp::Times(Box::new(Exp::Plus(Box::new(Exp::Var(String::from("iff"))), Box::new(Exp::Var(String::from("sskip"))))),Box::new(Exp::Var(String::from("doo"))))), Box::new(Exp::Var(String::from("thenn"))))))))
    }

    #[test]
    fn test66() {
        assert_eq!(parser::parse(&VecDeque::from([While, Id(String::from("a")), And, Id(String::from("b")), And, Id(String::from("c")), Do, Skip, Dollar])), Ok(Stmt::While(Box::new(Exp::And(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::And(Box::new(Exp::Var(String::from("b"))), Box::new(Exp::Var(String::from("c"))))))), Box::new(Stmt::Skip))))
    }

    #[test]
    fn test77() {
        assert_eq!(parser::parse(&VecDeque::from([While, Id(String::from("a")), Or, Id(String::from("b")), Or, Id(String::from("c")), Do, Skip, Dollar])), Ok(Stmt::While(Box::new(Exp::Or(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::Or(Box::new(Exp::Var(String::from("b"))), Box::new(Exp::Var(String::from("c"))))))), Box::new(Stmt::Skip))))
    }

    #[test]
    fn test88() {
        assert_eq!(parser::parse(&VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Plus, Id(String::from("b")), Plus, Id(String::from("c")), Dollar])), Ok(Stmt::Assn(String::from("x"), Box::new(Exp::Plus(Box::new(Exp::Plus(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::Var(String::from("b"))))),Box::new(Exp::Var(String::from("c"))))))))
    }

    #[test]
    fn test99() {
        assert_eq!(parser::parse(&VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Minus, Id(String::from("b")), Minus, Id(String::from("c")), Dollar])), Ok(Stmt::Assn(String::from("x"), Box::new(Exp::Minus(Box::new(Exp::Minus(Box::new(Exp::Var(String::from("a"))), Box::new(Exp::Var(String::from("b"))))),Box::new(Exp::Var(String::from("c"))))))))
    }

    #[test]
    fn test_all() {
        assert_eq!("This does not actually test anything", "");

        let test_strings = ["myVar <- x * (foo + bar)",
                            "while true do skip",
                            "if x <= 3 then x <- x - 1 else y <- y + 1",
                            "while y + 3 < 2 do y <- y + 1; x <- 0",
                            "y <- y + 1; x <- 0;",
                            "whiley <- (iff + sskip) * doo - thenn",
                            "while a && b && c do skip",
                            "while a || b || c do skip",
                            "x <- a + b + c",
                            "x <- a - b - c",
                        ];

        for s in test_strings {
            println!("String: {}", s);
            let ts = lexer::lex(s);
            match ts {
                Ok(ref a) => {
                    println!("Tokens: {:?}", a);
                    let r = parser::parse(a);
                    match r {
                        Ok(ref b) => println!("AST: {:?}\n", b),
                        Err(ref b) => println!("{:?}\n", b),
                    }
                },
                Err(ref a) => println!("{:?}", a),
            };
        }
    }
}
