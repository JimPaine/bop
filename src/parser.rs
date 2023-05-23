use crate::models::{Token, TokenType};
pub struct Parser {
    tokens: Vec<Token>
}

pub enum Expression {
    Assignment(Box<Expression>, Box<Expression>),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i32)
}

impl Expression {
    pub fn name(&self) -> String {
        match &self {
            Expression::Identifier(s) => s.to_string(),
            Expression::StringLiteral(s) => s.to_string(),
            Expression::NumberLiteral(i) => i.to_string(),
            _ => panic!("Expression not implemented")
        }
    }
}

pub trait Visitor {
    fn visit_token(&self, token: &Token, current_index: usize) -> Expression;
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();
        for i in 0..self.tokens.len() {
            match self.tokens[i].token_type {
                TokenType::ASSIGN => {
                    if i == 0 { panic!("First token can not be an assignment token!"); }
                    let ref t = self.tokens[i];
                    expressions.push(self.visit_token(t, i));
                },
                _ => { },
            }
        }
        expressions
    }

    fn error_on_no_match(actual: &TokenType, allowed: Vec<TokenType>, error_details: &str) {
        if !allowed.contains(actual) {
            panic!("Token type of {:?} is not allowed {}", actual, error_details);
        }
    }

}

impl Visitor for Parser {
    fn visit_token(&self, token: &Token, current_index: usize) -> Expression {
        match token.token_type {
            TokenType::ASSIGN => {

                let ref lhs = self.tokens[current_index - 1];
                Self::error_on_no_match(&lhs.token_type, vec![{TokenType::IDENTIFIER}], "on the left hand side of an assignment");


                let ref rhs = self.tokens[current_index + 1];
                Self::error_on_no_match(&rhs.token_type, vec![{TokenType::IDENTIFIER}, {TokenType::STRING}, {TokenType::NUMBER}], "on the right hand side of an assignment");

                Expression::Assignment(
                    Box::new(self.visit_token(lhs, current_index)),
                    Box::new(self.visit_token(rhs, current_index)))
            },
            TokenType::IDENTIFIER => Expression::Identifier(token.lexeme.clone()),
            TokenType::STRING => Expression::StringLiteral(token.lexeme.clone()),
            TokenType::NUMBER => match token.lexeme.parse::<i32>() {
                Ok(i) => Expression::NumberLiteral(i),
                Err(e) => panic!("Value is not valid for token type of NUMBER - {}", e),
            },
            _ => panic!("Token Type of {:?} is not implemented in the parser", token.token_type)
        }
    }
}