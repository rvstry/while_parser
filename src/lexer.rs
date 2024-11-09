use std::collections::VecDeque;
use crate::token::Token;
use crate::error::LexError;

fn peek(input: &str) -> Option<char> {
    input.chars().next()

}
fn eat(input: &str, c: char) -> Result<&str, LexError> {
    if c == peek(input).ok_or(LexError::WhileError)? {
        // consume
        Ok(input.split_at_checked(1).ok_or(LexError::WhileError)?.1)
    }
    else {Err(LexError::WhileError)}

}
fn is_more(input: &str) -> bool {
    !(input.is_empty())
}

fn lex_number(input: &str) -> Result<(Token, &str), LexError> {
    let mut lexeme = String::new();
    let mut i = input;
    while is_more(&i) {
        let c = peek(&i).ok_or(LexError::WhileError)?;
        if c.is_ascii_digit() {
            i = eat(&i, c)?;
            lexeme.push(c);
        }
        else {break};
    }

    Ok((Token::Num(lexeme), i))

}
fn lex_kw_or_id(input: &str) -> Result<(Token, &str), LexError> {
    let mut lexeme = String::new();
    let mut i = input;
    while is_more(&i) {
        let c = peek(&i).ok_or(LexError::WhileError)?;
        if c.is_ascii_alphanumeric() || c == '\'' {
            i = eat(&i, c)?;
            lexeme.push(c);
        }
        else {break};
    }

    let t = match lexeme.as_str() {
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "while" => Token::While,
        "do" => Token::Do,
        "skip" => Token::Skip,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Id(lexeme)

    };
    Ok((t, i))

}

pub fn lex(s: &str) -> Result<VecDeque<Token>, LexError> {
    let mut tokens: VecDeque<Token> = VecDeque::new();

    let mut i = s;
    while is_more(&i) {
        let c = peek(&i).ok_or(LexError::WhileError)?;

        match c as char {
            '=' => {i = eat(&i, '=')?; tokens.push_back(Token::Equals)},
            '!' => {i = eat(&i, '!')?; tokens.push_back(Token::Not)},
            '+' => {i = eat(&i, '+')?; tokens.push_back(Token::Plus)},
            '-' => {i = eat(&i, '-')?; tokens.push_back(Token::Minus)},
            '&' => {i = eat(&i, '&')?; i = eat(&i, '&')?; tokens.push_back(Token::And)}
            '|' => {i = eat(&i, '|')?; i = eat(&i, '|')?; tokens.push_back(Token::Or)}
            '<' => {
                i = eat(&i, '<')?;
                if peek(&i).ok_or(LexError::WhileError)? as char == '-' {
                    i = eat(&i, '-')?;
                    tokens.push_back(Token::Assignment)
                }
                else {tokens.push_back(Token::LessThan)}
            }
            '*' => {i = eat(&i, '*')?; tokens.push_back(Token::Asterisk)},
            '(' => {i = eat(&i, '(')?; tokens.push_back(Token::LeftParenthesis)},
            ')' => {i = eat(&i, ')')?; tokens.push_back(Token::RightParenthesis)},
            '{' => {i = eat(&i, '{')?; tokens.push_back(Token::LeftCurly)},
            '}' => {i = eat(&i, '}')?; tokens.push_back(Token::RightCurly)},
            ';' => {i = eat(&i, ';')?; tokens.push_back(Token::Semicolon)},
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

    Ok(tokens)
}
