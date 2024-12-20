mod ast;
mod error;
mod lexer;
mod parser;
mod token;

mod denotational;
mod operational;

fn main() {
    let tokens = lexer::lex("y <- x; while !(y = 1) do {y <- y - 1; x <- y * x}").unwrap();
    let ast = parser::parse(&tokens).unwrap();
    let mut state = denotational::State::new();
    state.update_var("x".to_string(), 5);

    println!("Initial state: {:?}", state);
    operational::execute_statement(&ast, &mut state).unwrap();
    println!("Final state: {:?}", state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_a_real_test() {
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
            let ts = lexer::lex(s);
            match ts {
                Ok(ref a) => {
                    println!("Tokens: {:?}", a);
                    let r = parser::parse(a);
                    match r {
                        Ok(ref b) => println!("AST: {:?}\n", b),
                        Err(ref b) => println!("{:?}\n", b),
                    }
                },
                Err(ref a) => println!("{:?}", a),
            };
        }
    }
}
