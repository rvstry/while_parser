mod ast;
mod error;
mod lexer;
mod parser;
mod token;

fn main() {
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
        let ts = lexer::lex(s.as_bytes().to_vec().into());
        match ts {
            Ok(ref a) => {
                println!("LEX SUCCESS!\nTokens: {:?}", a);
                let r = parser::parse(a);
                match r {
                    Ok(ref b) => println!("PARSE SUCCESS!\nAST: {:?}\n", b),
                    Err(ref b) => println!("PARSE FAIL!\n{:?}\n", b),
                }
            },
            Err(ref a) => println!("LEX FAIL!\n{:?}", a),
        };
    }
}
