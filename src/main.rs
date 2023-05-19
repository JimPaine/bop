mod lexer;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("a = b"));

    for token in lexer.scan().iter() {
        println!("{} {:?} {}", token.lexeme, token.token_type, token.line);
    }


}
