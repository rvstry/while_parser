use std::collections::VecDeque;
use crate::token::Token;

struct Recogniser {
    input: VecDeque<Token>,
}

impl Recogniser {

    fn init(s: VecDeque<Token>) -> Self {
        Self {input: s,}
    }


    fn peek(&self) -> Option<Token> {
        self.input.front().cloned()

    }

    fn eat(&mut self, tk: Token) {
        if tk == self.peek().unwrap() {
            // consume
            let mut p = self.input.clone();
            self.input = p.split_off(1);
        }
        else {panic!()}

    }

    fn parse_prog(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::If => { self.parse_stmt(); self.parse_stmts(); self.eat(Token::Dollar)},
            Token::While => { self.parse_stmt(); self.parse_stmts(); self.eat(Token::Dollar)},
            Token::Skip => { self.parse_stmt(); self.parse_stmts(); self.eat(Token::Dollar)},
            Token::Id(_) => { self.parse_stmt(); self.parse_stmts(); self.eat(Token::Dollar)},
            Token::LeftCurly => { self.parse_stmt(); self.parse_stmts(); self.eat(Token::Dollar)},
            _ => panic!(),
        };
    }

    fn parse_stmt(&mut self)  {
        let c = self.peek().unwrap();
        match c {
            Token::If => {self.eat(Token::If); self.parse_bexp(); self.eat(Token::Then); self.parse_stmt(); self.eat(Token::Else); self.parse_stmt()},
            Token::While => {self.eat(Token::While); self.parse_bexp(); self.eat(Token::Do); self.parse_stmt()},
            Token::Skip => {self.eat(Token::Skip)},
            Token::Id(s) => {self.eat(Token::Id(s)); self.eat(Token::Assignment); self.parse_aexp()},
            Token::LeftCurly => {self.eat(Token::LeftCurly); self.parse_stmt();self.parse_stmts(); self.eat(Token::RightCurly)},
            _ => panic!(),

        };
    }


    fn parse_stmts(&mut self)  {
        let c = self.peek().unwrap();
        match c {
            Token::Dollar => {},
            Token::RightCurly => {},
            Token::Semicolon => {self.eat(Token::Semicolon); self.parse_stmt(); self.parse_stmts();},
            _ => panic!(),

        };
    }


    fn parse_bexp(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(_) => {self.parse_bfac(); self.parse_bexps()},
            Token::Not => {self.parse_bfac(); self.parse_bexps()},
            Token::Num(_) => {self.parse_bfac(); self.parse_bexps()},
            Token::True => {self.parse_bfac(); self.parse_bexps()},
            Token::False => {self.parse_bfac(); self.parse_bexps()},
            Token::LeftParenthesis => {self.parse_bfac(); self.parse_bexps()},
            _ => panic!(),

        };
    }

    fn parse_bexps(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Then => {},
            Token::Do => {},
            Token::Or => {self.eat(Token::Or); self.parse_bfac(); self.parse_bexps()},
            Token::RightParenthesis => {},
            _ => panic!(),

        };
    }


    fn parse_bfac(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(_) => {self.parse_bneg(); self.parse_bfacs()},
            Token::Not => {self.parse_bneg(); self.parse_bfacs()},
            Token::Num(_) => {self.parse_bneg();self.parse_bfacs()},
            Token::True => {self.parse_bneg(); self.parse_bfacs()},
            Token::False => {self.parse_bneg(); self.parse_bfacs()},
            Token::LeftParenthesis => {self.parse_bneg(); self.parse_bfacs()},
            _ => panic!(),

        };
    }

    fn parse_bfacs(&mut self)  {
        let c = self.peek().unwrap();
        match c {
            Token::Then => {},
            Token::Do => {},
            Token::Or => {},
            Token::And => {self.eat(Token::And); self.parse_bneg(); self.parse_bfacs()},
            Token::RightParenthesis => {},
            _ => panic!(),

        };
    }

    fn parse_bneg(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(_) => {self.parse_brel()},
            Token::Not => {self.eat(Token::Not); self.parse_bneg()},
            Token::Num(_) => {self.parse_brel()},
            Token::True => {self.parse_brel()},
            Token::False => {self.parse_brel()},
            Token::LeftParenthesis => {self.parse_brel()},
            _ => panic!(),

        };
    }


    fn parse_brel(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(_) => {self.parse_aexp(); self.parse_brels()},
            Token::Num(_) => {self.parse_aexp(); self.parse_brels()},
            Token::True => {self.parse_aexp(); self.parse_brels()},
            Token::False => {self.parse_aexp(); self.parse_brels()},
            Token::LeftParenthesis => {self.parse_aexp(); self.parse_brels()},
            _ => panic!(),

        };
    }


    fn parse_brels(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Then => {},
            Token::Do => {},
            Token::Or => {},
            Token::And => {},
            Token::LessThan => {self.eat(Token::LessThan); self.parse_aexp()},
            Token::Equals => {self.eat(Token::Equals); self.parse_aexp()},
            Token::RightParenthesis => {},
            _ => panic!(),

        };
    }


    fn parse_aexp(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(_) => {self.parse_afac(); self.parse_aexps()},
            Token::Num(_) => {self.parse_afac(); self.parse_aexps()},
            Token::True => {self.parse_afac(); self.parse_aexps()},
            Token::False => {self.parse_afac(); self.parse_aexps()},
            Token::LeftParenthesis => {self.parse_afac(); self.parse_aexps()},
            _ => panic!(),

        };
    }


    fn parse_aexps(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Dollar => {},
            Token::Then =>{},
            Token::Else => {},
            Token::Do => {},
            Token::RightCurly => {},
            Token::Semicolon => {},
            Token::Or => {},
            Token::And => {},
            Token::LessThan => {},
            Token::Equals => {},
            Token::Plus => {self.eat(Token::Plus);self.parse_afac(); self.parse_aexps()},
            Token::Minus => {self.eat(Token::Minus); self.parse_afac(); self.parse_aexps()},
            Token::RightParenthesis => {},
            _ => panic!(),

        };
    }

    fn parse_afac(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(_) => {self.parse_atom(); self.parse_afacs()},
            Token::Num(_) => {self.parse_atom(); self.parse_afacs()},
            Token::True => {self.parse_atom(); self.parse_afacs()},
            Token::False => {self.parse_atom(); self.parse_afacs()},
            Token::LeftParenthesis => {self.parse_atom(); self.parse_afacs()},
            _ => panic!(),

        };
    }

    fn parse_afacs(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Dollar => {},
            Token::Then => {},
            Token::Else => {},
            Token::Do => {},
            Token::RightCurly => {},
            Token::Semicolon => {},
            Token::Or => {},
            Token::And => {},
            Token::LessThan =>{},
            Token::Equals =>{},
            Token::Plus => {},
            Token::Minus => {},
            Token::Asterisk => {self.eat(Token::Asterisk); self.parse_atom(); self.parse_afacs()},
            Token::RightParenthesis => {},
            _ => panic!(),

        };
    }
    fn parse_atom(&mut self) {
        let c = self.peek().unwrap();
        match c {
            Token::Id(s) => {self.eat(Token::Id(s))},
            Token::Num(s) => {self.eat(Token::Num(s))},
            Token::True => {self.eat(Token::True)},
            Token::False => {self.eat(Token::False)},
            Token::LeftParenthesis => {self.eat(Token::LeftParenthesis); self.parse_bexp(); self.eat(Token::RightParenthesis)},
            _ => panic!(),

        };
}
}

pub fn recognise(input: &VecDeque<Token>) {
    let mut recogniser = Recogniser::init(input.clone());
    recogniser.parse_prog(); // all programs must start with prog
}
