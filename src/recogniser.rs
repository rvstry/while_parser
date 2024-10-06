use std::collections::VecDeque;
use crate::token::Token;

fn peek(input: &VecDeque<Token>) -> Option<Token> {
    input.front().cloned()

}

fn eat(input: &VecDeque<Token>, tk: Token) ->VecDeque<Token>{
    if tk == peek(&input).unwrap() {
        // consume
        let mut p = input.clone();
        p.split_off(1)
    }
    else {panic!()}

}

fn parse_prog(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::If => { out = parse_stmt(input); out = parse_stmts(&out); eat(&out, Token::Dollar)},
        Token::While => { out = parse_stmt(input); out = parse_stmts(&out); eat(&out, Token::Dollar)},
        Token::Skip => { out = parse_stmt(input); out = parse_stmts(&out); eat(&out, Token::Dollar)},
        Token::Id(_) => { out = parse_stmt(input); out = parse_stmts(&out); eat(&out, Token::Dollar)},
        Token::LeftCurly => { out = parse_stmt(input); out = parse_stmts(&out); eat(&out, Token::Dollar)},
        _ => panic!(),
    };
    out // temporary!!
}

fn parse_stmt(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::If => {out = eat(input, Token::If); out = parse_bexp(&out); out = eat(&out, Token::Then); out = parse_stmt(&out); out = eat(&out, Token::Else); out = parse_stmt(&out)},
        Token::While => {out = eat(input, Token::While); out = parse_bexp(&out); out = eat(&out, Token::Do); out = parse_stmt(&out)},
        Token::Skip => {out = eat(input, Token::Skip)},
        Token::Id(s) => {out = eat(input, Token::Id(s)); out = eat(&out, Token::Assignment); out = parse_aexp(&out)},
        Token::LeftCurly => {out = eat(input, Token::LeftCurly); out = parse_stmt(&out); out = parse_stmts(&out); out = eat(&out, Token::RightCurly)},
        _ => panic!(),

    };

    out// temporary!!
}


fn parse_stmts(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Dollar => {out = input.clone()},
        Token::RightCurly => {out = input.clone()},
        Token::Semicolon => {out = eat(input, Token::Semicolon); out = parse_stmt(&out); out = parse_stmts(&out)},
        _ => panic!(),

    };

    out // temporary!!
}


fn parse_bexp(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(_) => {out = parse_bfac(input); out = parse_bexps(&out)},
        Token::Not => {out = parse_bfac(input); out = parse_bexps(&out)},
        Token::Num(_) => {out = parse_bfac(input); out = parse_bexps(&out)},
        Token::True => {out = parse_bfac(input); out = parse_bexps(&out)},
        Token::False => {out = parse_bfac(input); out = parse_bexps(&out)},
        Token::LeftParenthesis => {out = parse_bfac(input); out = parse_bexps(&out)},
        _ => panic!(),

    };

    out // temporary!!
}

fn parse_bexps(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Then => {out = input.clone()},
        Token::Do => {out = input.clone()},
        Token::Or => {out = eat(input, Token::Or); out = parse_bfac(&out); out = parse_bexps(&out)},
        Token::RightParenthesis => {out = input.clone()},
        _ => panic!(),

    };

    out // temporary!!
}


fn parse_bfac(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(_) => {out = parse_bneg(input); out = parse_bfacs(&out)},
        Token::Not => {out = parse_bneg(input); out = parse_bfacs(&out)},
        Token::Num(_) => {out = parse_bneg(input); out = parse_bfacs(&out)},
        Token::True => {out = parse_bneg(input); out = parse_bfacs(&out)},
        Token::False => {out = parse_bneg(input); out = parse_bfacs(&out)},
        Token::LeftParenthesis => {out = parse_bneg(input); out = parse_bfacs(&out)},
        _ => panic!(),

    };

    out // temporary!!
}

fn parse_bfacs(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Then => {out = input.clone()},
        Token::Do => {out = input.clone()},
        Token::Or => {out = input.clone()},
        Token::And => {out = eat(input, Token::And); out = parse_bneg(&out); out = parse_bfacs(&out)},
        Token::RightParenthesis => {out = input.clone()},
        _ => panic!(),

    };

    out // temporary!!
}

fn parse_bneg(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(_) => {out = parse_brel(input)},
        Token::Not => {out = eat(input, Token::Not); out = parse_bneg(&out)},
        Token::Num(_) => {out = parse_brel(input)},
        Token::True => {out = parse_brel(input)},
        Token::False => {out = parse_brel(input)},
        Token::LeftParenthesis => {out = parse_brel(input)},
        _ => panic!(),

    };

    out // temporary!!
}


fn parse_brel(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(_) => {out = parse_aexp(input); out = parse_brels(&out)},
        Token::Num(_) => {out = parse_aexp(input); out = parse_brels(&out)},
        Token::True => {out = parse_aexp(input); out = parse_brels(&out)},
        Token::False => {out = parse_aexp(input); out = parse_brels(&out)},
        Token::LeftParenthesis => {out = parse_aexp(input); out = parse_brels(&out)},
        _ => panic!(),

    };

    out // temporary!!
}


fn parse_brels(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Then => {out = input.clone()},
        Token::Do => {out = input.clone()},
        Token::Or => {out = input.clone()},
        Token::And => {out = input.clone()},
        Token::LessThan => {out = eat(input, Token::LessThan); out = parse_aexp(&out)},
        Token::Equals => {out = eat(input, Token::Equals); out = parse_aexp(&out)},
        Token::RightParenthesis => {out = input.clone()},
        _ => panic!(),

    };

    out // temporary!!
}


fn parse_aexp(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(_) => {out = parse_afac(input); out = parse_aexps(&out)},
        Token::Num(_) => {out = parse_afac(input); out = parse_aexps(&out)},
        Token::True => {out = parse_afac(input); out = parse_aexps(&out)},
        Token::False => {out = parse_afac(input); out = parse_aexps(&out)},
        Token::LeftParenthesis => {out = parse_afac(input); out = parse_aexps(&out)},
        _ => panic!(),

    };

    out // temporary!!
}


fn parse_aexps(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Dollar => {out = input.clone()},
        Token::Then =>{out = input.clone()},
        Token::Else => {out = input.clone()},
        Token::Do => {out = input.clone()},
        Token::RightCurly => {out = input.clone()},
        Token::Semicolon => {out = input.clone()},
        Token::Or => {out = input.clone()},
        Token::And => {out = input.clone()},
        Token::LessThan => {out = input.clone()},
        Token::Equals => {out = input.clone()},
        Token::Plus => {out = eat(input, Token::Plus); out = parse_afac(&out); out = parse_aexps(&out)},
        Token::Minus => {out = eat(input, Token::Minus); out = parse_afac(&out); out = parse_aexps(&out)},
        Token::RightParenthesis => {out = input.clone()},
        _ => panic!(),

    };

    out // temporary!!
}

fn parse_afac(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(_) => {out = parse_atom(input); out = parse_afacs(&out)},
        Token::Num(_) => {out = parse_atom(input); out = parse_afacs(&out)},
        Token::True => {out = parse_atom(input); out = parse_afacs(&out)},
        Token::False => {out = parse_atom(input); out = parse_afacs(&out)},
        Token::LeftParenthesis => {out = parse_atom(input); out = parse_afacs(&out)},
        _ => panic!(),

    };

    out // temporary!!
}

fn parse_afacs(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Dollar => {out = input.clone()},
        Token::Then => {out = input.clone()},
        Token::Else => {out = input.clone()},
        Token::Do => {out = input.clone()},
        Token::RightCurly => {out = input.clone()},
        Token::Semicolon => {out = input.clone()},
        Token::Or => {out = input.clone()},
        Token::And => {out = input.clone()},
        Token::LessThan =>{out = input.clone()},
        Token::Equals =>{out = input.clone()},
        Token::Plus => {out = input.clone()},
        Token::Minus => {out = input.clone()},
        Token::Asterisk => {out = eat(input, Token::Asterisk); out = parse_atom(&out); out = parse_afacs(&out)},
        Token::RightParenthesis => {out = input.clone()},
        _ => panic!(),

    };

    out // temporary!!
}
fn parse_atom(input: &VecDeque<Token>) ->VecDeque<Token> {
    let c = peek(&input).unwrap();
    let mut out = VecDeque::new();
    match c {
        Token::Id(s) => {out = eat(input, Token::Id(s))},
        Token::Num(s) => {out = eat(input, Token::Num(s))},
        Token::True => {out = eat(input, Token::True)},
        Token::False => {out = eat(input, Token::False)},
        Token::LeftParenthesis => {out =  eat(input, Token::LeftParenthesis); out = parse_bexp(&out); out = eat(&out, Token::RightParenthesis)},
        _ => panic!(),

    };

    out // temporary!!
}

pub fn recognise(input: &VecDeque<Token>) {
    parse_prog(input);
}
