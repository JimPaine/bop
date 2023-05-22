mod lexer;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("abc = x\ndef = xyz"));

    for token in lexer.scan().iter() {
        println!("{} {:?} {}", token.lexeme, token.token_type, token.line);
    }


}
