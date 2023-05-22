use crate::lexer::{Lexer, TokenType};


#[test]
fn scan_identifier() {
    // arrange
    let mut lexer = Lexer::new("abc");

    // act
    let tokens = lexer.scan();

    // assert
    assert!(matches!(tokens[0].token_type, TokenType::IDENTIFIER));
    assert_eq!(tokens[0].lexeme, "abc");
    assert_eq!(tokens[0].line, 1);
}

#[test]
fn scan_assign() {
    // arrange
    let mut lexer = Lexer::new("=");

    // act
    let tokens = lexer.scan();

    // assert
    assert!(matches!(tokens[0].token_type, TokenType::ASSIGN));
    assert_eq!(tokens[0].lexeme, "=");
    assert_eq!(tokens[0].line, 1);
}

#[test]
fn scan_eof() {
    // arrange
    let mut lexer = Lexer::new(" ");

    // act
    let tokens = lexer.scan();

    // assert
    assert!(matches!(tokens[0].token_type, TokenType::EOF));
    assert_eq!(tokens[0].lexeme, "\0");
    assert_eq!(tokens[0].line, 1);
}

#[test]
fn scan_ignores_whitespace() {
    // arrange
    let mut lexer = Lexer::new("abc =");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0].token_type, TokenType::IDENTIFIER));
    assert!(matches!(tokens[1].token_type, TokenType::ASSIGN));
    assert!(matches!(tokens[2].token_type, TokenType::EOF));
}

#[test]
fn scan_short_identifier() {
    // arrange
    let mut lexer = Lexer::new("a = b");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].lexeme, "a");
    assert_eq!(tokens[2].lexeme, "b");
}

#[test]
fn scan_dot() {
    // arrange
    let mut lexer = Lexer::new("a.b = x");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 6);
    assert!(matches!(tokens[1].token_type, TokenType::DOT));
}