mod lexer;
mod parser;
mod ast;
mod error;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source_code = r#"
        let x = 10 ;
        let y = 0 ;
        
        if (x > 5) {
            y = 1 ;
        } else {
            y = 2 ;
        }

        while (y < 5) {
            y = y + 1 ;
        }

        do {
            y = y - 1 ;
        } while (y > 0) ;

        fn add(a, b) {
            return a + b ;
        }

        let z = add(x, y) ;

        let w = 100
    "#;

    // Lexer needs to be mutable for tokenize
    let mut lexer = Lexer::new(source_code);

    // Tokenize source code with error handling
    let tokens_result = lexer.tokenize();

    match tokens_result {
        Ok(tokens) => {
            println!("Tokens:");
            for token in &tokens {
                println!("{:?}", token);
            }
            println!("");

            // Create parser with tokens
            let mut parser = Parser::new(tokens);

            match parser.parse_program() {
                Ok(ast) => {
                    println!("AST:");
                    for stmt in &ast {
                        println!("{:#?}", stmt);
                    }
                }
                Err(e) => {
                    println!("Parser error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Lexer error: {}", e);
        }
    }
}