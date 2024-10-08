mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    println!("Hello, world!");
    println!("{:?}", lexer::lex("myVar <- x * (foo + bar)".as_bytes().to_vec().into()));
    // println!("{:?}", lexer::lex("while true do skip".as_bytes().to_vec().into()));
    parser::recognise(&lexer::lex("myVar <- x * (foo + bar)".as_bytes().to_vec().into()));
    // parser::recognise(&lexer::lex("while true do skip".as_bytes().to_vec().into()));
    // parser::recognise(&lexer::lex("if x <= 3 then x <- x - 1 else y <- y + 1".as_bytes().to_vec().into()));
    // parser::recognise(&lexer::lex("while y + 3 < 2 do y <- y + 1; x <- 0".as_bytes().to_vec().into()));
    // parser::recognise(&lexer::lex("y <- y + 1; x <- 0;".as_bytes().to_vec().into()));
    // parser::recognise(&lexer::lex("whiley <- (iff + sskip) * doo - thenn".as_bytes().to_vec().into()));
}
