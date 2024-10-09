use std::collections::VecDeque;
use crate::token::Token;
use crate::error::LexError;

fn peek(input: &VecDeque<u8>) -> Option<u8> {
    input.front().copied()

}
fn eat(input: &VecDeque<u8>, c: u8) -> Result<VecDeque<u8>, LexError> {
    if c == peek(input).unwrap() {
        // consume
        let mut p = input.clone();
        Ok(p.split_off(1))
    }
    else {Err(LexError::WhileError)}

}
fn is_more(input: &VecDeque<u8>) -> bool {
    !(input.is_empty())
}

fn lex_number(input: &VecDeque<u8>) -> Result<(Token, VecDeque<u8>), LexError> {
    let mut lexeme: VecDeque<u8> = VecDeque::new();
    let mut i = input.to_owned();
    while is_more(&i) {
        let c = peek(&i).unwrap();
        if c.is_ascii_digit() {
            i = eat(&i, c)?;
            lexeme.push_back(c);
        }
        else {break};
    }

    Ok((Token::Num(String::from_utf8(lexeme.into()).unwrap()), i))

}
fn lex_kw_or_id(input: &VecDeque<u8>) -> Result<(Token, VecDeque<u8>), LexError> {
    let mut lexeme: VecDeque<u8> = VecDeque::new();
    let mut i = input.to_owned();
    while is_more(&i) {
        let c = peek(&i).unwrap();
        if c.is_ascii_alphanumeric() || c == 0x27 {
            i = eat(&i, c)?;
            lexeme.push_back(c);
        }
        else {break};
    }

    let t = match String::from_utf8(lexeme.to_owned().into()).unwrap().as_str() {
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "while" => Token::While,
        "do" => Token::Do,
        "skip" => Token::Skip,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Id(String::from_utf8(lexeme.to_owned().into()).unwrap())

    };
    Ok((t, i))

}

pub fn lex(s: VecDeque<u8>) -> Result<VecDeque<Token>, LexError> {
    let mut tokens: VecDeque<Token> = VecDeque::new();

    let mut i = s.to_owned();
    while is_more(&i) {
        let x = peek(&i);
        let c = match x {
            Some(a) => a,
            None => break
        };

        match c as char {
            '=' => {i = eat(&i, b'=')?; tokens.push_back(Token::Equals)},
            '!' => {i = eat(&i, b'!')?; tokens.push_back(Token::Not)},
            '+' => {i = eat(&i, b'+')?; tokens.push_back(Token::Plus)},
            '-' => {i = eat(&i, b'-')?; tokens.push_back(Token::Minus)},
            '&' => {i = eat(&i, b'&')?; i = eat(&i, b'&')?; tokens.push_back(Token::And)}
            '|' => {i = eat(&i, b'|')?; i = eat(&i, b'|')?; tokens.push_back(Token::Or)}
            '<' => {
                i = eat(&i, b'<')?;
                if peek(&i).unwrap() as char == '-' {
                    i = eat(&i, b'-')?;
                    tokens.push_back(Token::Assignment)
                }
                else {tokens.push_back(Token::LessThan)}
            }
            '*' => {i = eat(&i, b'*')?; tokens.push_back(Token::Asterisk)},
            '(' => {i = eat(&i, b'(')?; tokens.push_back(Token::LeftParenthesis)},
            ')' => {i = eat(&i, b')')?; tokens.push_back(Token::RightParenthesis)},
            '{' => {i = eat(&i, b'{')?; tokens.push_back(Token::LeftCurly)},
            '}' => {i = eat(&i, b'}')?; tokens.push_back(Token::RightCurly)},
            ';' => {i = eat(&i, b';')?; tokens.push_back(Token::Semicolon)},
            _ => {
                if c.is_ascii_digit() {
                    let (t, out) = lex_number(&i)?;
                    i = out;
                    tokens.push_back(t);
                }
                if c.is_ascii_lowercase() {
                    let (t, out) = lex_kw_or_id(&i)?;
                    i = out;
                    tokens.push_back(t);
                }
                if c.is_ascii_whitespace() {
                    i = eat(&i, c)?;
                }
            }


        }


    }
    tokens.push_back(Token::Dollar);
    println!("{:?}", tokens);

    Ok(tokens)
}
