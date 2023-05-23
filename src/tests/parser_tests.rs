use crate::parser::{Parser, Expression};
use crate::models::{Token, TokenType};

#[test]
fn parse_property_assigned_property() {
    // arrange
    let tokens = vec![
        {Token { token_type: TokenType::IDENTIFIER, lexeme: "a".to_string(), line: 1 }},
        {Token { token_type: TokenType::ASSIGN, lexeme: "=".to_string(), line: 1 }},
        {Token { token_type: TokenType::IDENTIFIER, lexeme: "b".to_string(), line: 1 }},
        {Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: 1 }},
    ];
    let mut parser = Parser::new(tokens);

    // act
    let expressions = parser.parse();

    // assert
    assert!(
        matches!(&expressions[0], Expression::Assignment(l, r)
            if matches!(&*l.name(), "a")
            && matches!(&*r.name(), "b")
        )
    );
}

#[test]
fn parse_property_assigned_string() {
    // arrange
    let tokens = vec![
        {Token { token_type: TokenType::IDENTIFIER, lexeme: "a".to_string(), line: 1 }},
        {Token { token_type: TokenType::ASSIGN, lexeme: "=".to_string(), line: 1 }},
        {Token { token_type: TokenType::STRING, lexeme: "b".to_string(), line: 1 }},
        {Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: 1 }},
    ];
    let mut parser = Parser::new(tokens);

    // act
    let expressions = parser.parse();

    // assert
    assert!(
        matches!(&expressions[0], Expression::Assignment(l, r)
            if matches!(&*l.name(), "a")
            && matches!(&*r.name(), "b")
        )
    );
}

#[test]
fn parse_property_assigned_number() {
    // arrange
    let tokens = vec![
        {Token { token_type: TokenType::IDENTIFIER, lexeme: "a".to_string(), line: 1 }},
        {Token { token_type: TokenType::ASSIGN, lexeme: "=".to_string(), line: 1 }},
        {Token { token_type: TokenType::NUMBER, lexeme: "123".to_string(), line: 1 }},
        {Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: 1 }},
    ];
    let mut parser = Parser::new(tokens);

    // act
    let expressions = parser.parse();

    // assert
    assert!(
        matches!(&expressions[0], Expression::Assignment(l, r)
            if matches!(&*l.name(), "a")
            && matches!(&*r.name(), "123")
        )
    );
}

#[test]
#[should_panic(expected = "Token type of EOF is not allowed on the right hand side of an assignment")]
fn parse_panic_rhs_is_wrong_type() {
    // arrange
    let tokens = vec![
        {Token { token_type: TokenType::IDENTIFIER, lexeme: "a".to_string(), line: 1 }},
        {Token { token_type: TokenType::ASSIGN, lexeme: "=".to_string(), line: 1 }},
        {Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: 1 }},
    ];
    let mut parser = Parser::new(tokens);

    // act
    parser.parse();
}

#[test]
#[should_panic(expected = "Token type of STRING is not allowed on the left hand side of an assignment")]
fn parse_panic_lhs_is_wrong_type() {
    // arrange
    let tokens = vec![
        {Token { token_type: TokenType::STRING, lexeme: "a".to_string(), line: 1 }},
        {Token { token_type: TokenType::ASSIGN, lexeme: "=".to_string(), line: 1 }},
        {Token { token_type: TokenType::STRING, lexeme: "a".to_string(), line: 1 }},
        {Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: 1 }},
    ];
    let mut parser = Parser::new(tokens);

    // act
    parser.parse();
}


#[test]
#[should_panic(expected = "First token can not be an assignment token!")]
fn parse_panic_assign_is_first() {
    // arrange
    let tokens = vec![
        {Token { token_type: TokenType::ASSIGN, lexeme: "=".to_string(), line: 1 }},
        {Token { token_type: TokenType::IDENTIFIER, lexeme: "b".to_string(), line: 1 }},
        {Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: 1 }},
    ];
    let mut parser = Parser::new(tokens);

    // act
    parser.parse();
}