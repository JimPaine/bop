use crate::lexer::Lexer;
use crate::models::TokenType;

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

#[test]
fn scan_string() {
    // arrange
    let mut lexer = Lexer::new("a = \"hello world\"");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[2].token_type, TokenType::STRING));
    assert_eq!(tokens[2].lexeme, "hello world".to_string());
}

#[test]
#[should_panic(expected = "string on line 1 is not closed")]
fn scan_string_panic_when_not_closed() {
    // arrange
    let mut lexer = Lexer::new("a = \"hello world");

    // act
    lexer.scan();
}

#[test]
fn scan_number() {
    // arrange
    let mut lexer = Lexer::new("a = 123");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[2].token_type, TokenType::NUMBER));
    assert_eq!(tokens[2].lexeme, "123".to_string());
}

#[test]
fn scan_decimal_number() {
    // arrange
    let mut lexer = Lexer::new("a = 1.23");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[2].token_type, TokenType::NUMBER));
    assert_eq!(tokens[2].lexeme, "1.23".to_string());
}

#[test]
#[should_panic(expected = "number has more than one '.'")]
fn scan_only_one_decimal_number() {
    // arrange
    let mut lexer = Lexer::new("a = 1.2.3");

    // act
    lexer.scan();
}

#[test]
fn scan_negative_number() {
    // arrange
    let mut lexer = Lexer::new("a = -123");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[2].token_type, TokenType::NUMBER));
    assert_eq!(tokens[2].lexeme, "-123".to_string());
}

#[test]
fn scan_assume_zero_point_number() {
    // arrange
    let mut lexer = Lexer::new("a = .23");

    // act
    let tokens = lexer.scan();

    // assert
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[2].token_type, TokenType::NUMBER));
    assert_eq!(tokens[2].lexeme, ".23".to_string());
}