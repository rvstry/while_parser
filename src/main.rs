mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    println!("Hello, world!");
    let test_strings = ["myVar <- x * (foo + bar)",
                        "while true do skip",
                        "if x <= 3 then x <- x - 1 else y <- y + 1",
                        "while y + 3 < 2 do y <- y + 1; x <- 0",
                        "y <- y + 1; x <- 0;",
                        "whiley <- (iff + sskip) * doo - thenn",
                       ];

    for s in test_strings {
        let ts = lexer::lex(s.as_bytes().to_vec().into());
        let r = parser::parse(&ts);
        match r {
            Ok(ref a) => println!("PARSE SUCCESS!\nString: \"{:?}\"\nTokens: {:?}\nAST: {:?}\n", s, ts, a),
            Err(ref a) => println!("PARSE FAIL!\nString: \"{:?}\"\nError: {:?}\n", s,/* r,*/ a),

        }
    }
}
