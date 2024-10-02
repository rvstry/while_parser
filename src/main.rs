mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
    println!("{:?}", lexer::Lexer::lex("myVar <- x * (foo + bar)".as_bytes().to_vec().into()))
}
