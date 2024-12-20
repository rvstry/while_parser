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

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test0() {
        assert_eq!(lex("myVar <- x * (foo + bar)"), Ok(VecDeque::from([Id(String::from("myVar")), Assignment, Id(String::from("x")), Asterisk, LeftParenthesis, Id(String::from("foo")), Plus, Id(String::from("bar")), RightParenthesis, Dollar])))
    }

    #[test]
    fn test1() {
        assert_eq!(lex("while true do skip"), Ok(VecDeque::from([While, True, Do, Skip, Dollar])))
    }

    #[test]
    fn test2() {
        assert_eq!(lex("if x <= 3 then x <- x - 1 else y <- y + 1"), Ok(VecDeque::from([If, Id(String::from("x")), LessThan, Equals, Num(String::from("3")), Then, Id(String::from("x")), Assignment, Id(String::from("x")), Minus, Num(String::from("1")), Else, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Dollar])))
    }

    #[test]
    fn test3() {
        assert_eq!(lex("while y + 3 < 2 do y <- y + 1; x <- 0"), Ok(VecDeque::from([While, Id(String::from("y")), Plus, Num(String::from("3")), LessThan, Num(String::from("2")), Do, Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Dollar])))
    }

    #[test]
    fn test4() {
        assert_eq!(lex("y <- y + 1; x <- 0;"), Ok(VecDeque::from([Id(String::from("y")), Assignment, Id(String::from("y")), Plus, Num(String::from("1")), Semicolon, Id(String::from("x")), Assignment, Num(String::from("0")), Semicolon, Dollar])))
    }

    #[test]
    fn test5() {
        assert_eq!(lex("whiley <- (iff + sskip) * doo - thenn"), Ok(VecDeque::from([Id(String::from("whiley")), Assignment, LeftParenthesis, Id(String::from("iff")), Plus, Id(String::from("sskip")), RightParenthesis, Asterisk, Id(String::from("doo")), Minus, Id(String::from("thenn")), Dollar])))
    }

    #[test]
    fn test6() {
        assert_eq!(lex("while a && b && c do skip"), Ok(VecDeque::from([While, Id(String::from("a")), And, Id(String::from("b")), And, Id(String::from("c")), Do, Skip, Dollar])))
    }

    #[test]
    fn test7() {
        assert_eq!(lex("while a || b || c do skip"), Ok(VecDeque::from([While, Id(String::from("a")), Or, Id(String::from("b")), Or, Id(String::from("c")), Do, Skip, Dollar])))
    }

    #[test]
    fn test8() {
        assert_eq!(lex("x <- a + b + c"), Ok(VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Plus, Id(String::from("b")), Plus, Id(String::from("c")), Dollar])))
    }

    #[test]
    fn test9() {
        assert_eq!(lex("x <- a - b - c"), Ok(VecDeque::from([Id(String::from("x")), Assignment, Id(String::from("a")), Minus, Id(String::from("b")), Minus, Id(String::from("c")), Dollar])))
    }
}
