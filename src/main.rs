use std::collections::VecDeque;

#[derive(Debug)]
enum Token {
    If,
    Then,
    Else,
    While,
    Do,
    Skip,
    Id(String),
    Plus,
    Minus,
    Asterisk,
    True,
    False,
    LessThan,
    Equals,
    And,
    Or,
    Not,
    Num(String),
    LeftParenthesis,
    RightParenthesis,
    LeftCurly,
    RightCurly,
    Semicolon,
    Assignment,
    Dollar,
}


struct Lexer {
    input: VecDeque<u8>,
}

impl Lexer {
    fn peek(&self) -> Option<u8> {
        self.input.get(0).copied()

    }
    fn eat(&mut self, c: u8) {
        if c == self.peek().unwrap() {
            // consume
            &mut self.input.pop_front();
        }

    }
    fn is_more(&self) -> bool {
        !(self.input.is_empty())
    }
    fn init(s: VecDeque<u8>) ->Self {
        Self {
            input: s
        }
    }

    fn lex_number(&mut self) ->Token {
        let mut lexeme: VecDeque<u8> = VecDeque::new();
        while self.peek().unwrap().is_ascii_digit() {
            let c = self.peek().unwrap();
            &mut self.eat(c);
            lexeme.push_back(c);
        }
        Token::Num(String::from_utf8(lexeme.into()).unwrap())

    }
    fn lex_kw_or_id(&mut self) ->Token {
        let mut lexeme: VecDeque<u8> = VecDeque::new();
        while self.peek().unwrap().is_ascii_alphanumeric() || (self.peek().unwrap() == 0x27) {
            let c = self.peek().unwrap();
            &mut self.eat(c);
            lexeme.push_back(c);
        }
        match String::from_utf8(lexeme.to_owned().into()).unwrap().as_str() {
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "while" => Token::While,
            "do" => Token::Do,
            "skip" => Token::Skip,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Id(String::from_utf8(lexeme.to_owned().into()).unwrap())

        }
    }

    fn lex(s: VecDeque<u8>) -> VecDeque<Token> {
        let mut lexer = Self::init(s);
        let mut tokens: VecDeque<Token> = VecDeque::new();

        while lexer.is_more() {
            let x = lexer.peek();
            let c = match x {
                Some(a) => a,
                None => break
            };

            match c as char {
                '=' => {lexer.eat('=' as u8); tokens.push_back(Token::Equals)},
                '!' => {lexer.eat('!' as u8); tokens.push_back(Token::Not)},
                '+' => {lexer.eat('+' as u8); tokens.push_back(Token::Plus)},
                '-' => {lexer.eat('-' as u8); tokens.push_back(Token::Minus)},
                '&' => {lexer.eat('&' as u8); lexer.eat('&' as u8); tokens.push_back(Token::And)}
                '|' => {lexer.eat('|' as u8); lexer.eat('|' as u8); tokens.push_back(Token::Or)}
                '<' => {
                    lexer.eat('<' as u8);
                    if lexer.peek().unwrap() as char == '-' {
                        lexer.eat('-' as u8);
                        tokens.push_back(Token::Assignment)
                    }
                    else {tokens.push_back(Token::LessThan)}
                }
                '*' => {lexer.eat('*' as u8); tokens.push_back(Token::Asterisk)},
                '(' => {lexer.eat('(' as u8); tokens.push_back(Token::LeftParenthesis)},
                ')' => {lexer.eat(')' as u8); tokens.push_back(Token::RightParenthesis)},

                '{' => {lexer.eat('{' as u8); tokens.push_back(Token::LeftCurly)},
                '}' => {lexer.eat('}' as u8); tokens.push_back(Token::RightCurly)},
                ';' => {lexer.eat(';' as u8); tokens.push_back(Token::Semicolon)},
                _ => {
                    if c.is_ascii_digit() {
                        tokens.push_back(lexer.lex_number());
                    }
                    if c.is_ascii_lowercase() {
                        tokens.push_back(lexer.lex_kw_or_id());
                    }
                    if c.is_ascii_whitespace() {
                        lexer.eat(c as u8);
                    }
                }


            }


        }
        tokens.push_back(Token::Dollar);

        tokens
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", Lexer::lex("myVar <- x * (foo + bar)".as_bytes().to_vec().into()))
}
