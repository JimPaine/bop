use std::{vec::IntoIter, iter::Peekable};

use crate::models::{Token, TokenType};
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>
}

pub struct PropertyExpression {
    pub name: String,
    pub child: Option<Box<PropertyExpression>>
}

impl ExpressionEvaluator for PropertyExpression {
    fn display(&self) -> String {
        self.name.clone()
    }
    fn typename(&self) -> String {
        String::from("PropertyExpression")
    }
}

pub struct AssignmentExpression {
    pub left: PropertyExpression,
    pub right: Box<dyn ExpressionEvaluator>
}

pub struct ConstantExpression<T> {
    pub value: T
}
impl ExpressionEvaluator for ConstantExpression<f32> {
    fn display(&self) -> String {
        self.value.to_string()
    }
    fn typename(&self) -> String {
        String::from("ConstantExpression<f32>")
    }
}

impl ExpressionEvaluator for ConstantExpression<String> {
    fn display(&self) -> String {
        self.value.to_string()
    }
    fn typename(&self) -> String {
        String::from("ConstantExpression<String>")
    }
}

pub trait ExpressionEvaluator {
    fn display(&self) -> String;
    fn typename(&self) -> String;
}

pub trait Visitor {
    fn visit_assignment(&mut self, lhs: PropertyExpression) -> AssignmentExpression;
    fn visit_identifier(&mut self, token: &Token) -> PropertyExpression;

    fn visit_string(&mut self, token: Token) -> ConstantExpression<String>;
    fn visit_number(&mut self, token: Token) -> ConstantExpression<f32>;
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable()
        }
    }

    pub fn parse(&mut self) -> Vec<AssignmentExpression> {
        let mut expressions = Vec::new();
        let mut lhs = None;

        while let Some(t) = self.tokens.next() {
            match t.token_type {
                TokenType::IDENTIFIER => {
                    lhs = Some(self.visit_identifier(&t))
                },
                TokenType::ASSIGN => {
                    match lhs {
                        Some(l) => {
                            expressions.push(self.visit_assignment(l));
                            lhs = None;
                        },
                        _ => panic!("First token can not be an assignment token!"),
                    }
                },
                TokenType::DOT => {},
                TokenType::EOL => {},
                TokenType::EOF => break,
                t => panic!("Token type of {:?} is not allowed on the left hand side of an assignment", t),
            }
        }

        expressions
    }
}

impl Visitor for Parser {
    fn visit_assignment(&mut self, lhs: PropertyExpression) -> AssignmentExpression {
        let rhs = match self.tokens.next() {
            Some(o) => o,
            _ => panic!("todo")
        };

        match rhs.token_type {
            TokenType::IDENTIFIER => AssignmentExpression { left: lhs, right: Box::new(self.visit_identifier(&rhs)) },
            TokenType::STRING => AssignmentExpression { left: lhs, right: Box::new(self.visit_string(rhs)) },
            TokenType::NUMBER => AssignmentExpression { left: lhs, right: Box::new(self.visit_number(rhs)) },
            _ => panic!("Token type of {:?} is not allowed on the right hand side of an assignment", rhs.token_type)
        }
    }

    fn visit_identifier(&mut self, token: &Token) -> PropertyExpression {
        let mut descendants = Vec::new();

        while let Some(x) = self.tokens.peek() {

            match x.token_type {
                TokenType::DOT => { self.tokens.next(); },
                TokenType::IDENTIFIER => {
                    match self.tokens.next() {
                        Some(c) => {
                            descendants.push(c);
                        },
                        _ => panic!("todo"),
                    }
                }
                _ => break,
            }
        }

        let mut child = None;
        for i in (0..descendants.len()).rev() {
            if i == descendants.len() -1 {
                child = Some(PropertyExpression { name: descendants[i].lexeme.clone(), child: None });
            } else {
                match child {
                    Some(c) => {
                        let parent = PropertyExpression { name: descendants[i].lexeme.clone(), child: Some(Box::new(c)) };
                        child = Some(parent);
                    },
                    _ => panic!("todo")
                }
            }
        }

        PropertyExpression { name: token.lexeme.clone(), child: match child {
            Some(c) => Some(Box::new(c)),
            _ => None,
        }}


    }

    fn visit_string(&mut self, token: Token) -> ConstantExpression<String> {
        ConstantExpression { value: token.lexeme.clone() }
    }

    fn visit_number(&mut self, token: Token) -> ConstantExpression<f32> {
        ConstantExpression::<f32> { value: match token.lexeme.parse::<f32>() {
            Ok(i) => i,
            Err(e) => panic!("Value is not valid for token type of NUMBER - {}", e),
        }}
    }
}