use crate::parser::{Parser};
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
    assert_eq!(expressions[0].left.name, "a");
    assert_eq!(expressions[0].right.display(), "b");
    assert_eq!(expressions[0].right.typename(), "PropertyExpression");
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
    assert_eq!(expressions[0].left.name, "a");
    assert_eq!(expressions[0].right.display(), "b");
    assert_eq!(expressions[0].right.typename(), "ConstantExpression<String>");
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
    assert_eq!(expressions[0].left.name, "a");
    assert_eq!(expressions[0].right.display(), "123");
    assert_eq!(expressions[0].right.typename(), "ConstantExpression<f32>");
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