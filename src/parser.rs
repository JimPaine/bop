use crate::models::{Token, TokenType};
pub struct Parser {
    tokens: Vec<Token>
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
    fn visit_assignment(&self, token: &Token, current_index: usize) -> AssignmentExpression;
    fn visit_identifier(&self, token: &Token) -> PropertyExpression;
    fn visit_string(&self, token: &Token) -> ConstantExpression<String>;
    fn visit_number(&self, token: &Token) -> ConstantExpression<f32>;
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens
        }
    }

    pub fn parse(&mut self) -> Vec<AssignmentExpression> {
        let mut expressions = Vec::new();
        for i in 0..self.tokens.len() {
            match self.tokens[i].token_type {
                TokenType::ASSIGN => {
                    if i == 0 { panic!("First token can not be an assignment token!"); }
                    let ref t = self.tokens[i];
                    expressions.push(self.visit_assignment(t, i));
                },
                _ => { },
            }
        }
        expressions
    }
}

impl Visitor for Parser {
    fn visit_assignment(&self, token: &Token, current_index: usize) -> AssignmentExpression {
        let ref lhs = self.tokens[current_index - 1];
        let left = match self.tokens[current_index - 1].token_type {
            TokenType::IDENTIFIER => self.visit_identifier(lhs),
            _ => panic!("Token type of {:?} is not allowed on the left hand side of an assignment", lhs.token_type)
        };

        let ref rhs = self.tokens[current_index + 1];

        match rhs.token_type {
            TokenType::IDENTIFIER => AssignmentExpression { left: left, right: Box::new(self.visit_identifier(rhs)) },
            TokenType::STRING => AssignmentExpression { left: left, right: Box::new(self.visit_string(rhs)) },
            TokenType::NUMBER => AssignmentExpression { left: left, right: Box::new(self.visit_number(rhs)) },
            _ => panic!("Token type of {:?} is not allowed on the right hand side of an assignment", rhs.token_type)
        }
    }

    fn visit_identifier(&self, token: &Token) -> PropertyExpression {
        PropertyExpression { name: token.lexeme.clone(), child: None }
    }

    fn visit_string(&self, token: &Token) -> ConstantExpression<String> {
        ConstantExpression { value: token.lexeme.clone() }
    }

    fn visit_number(&self, token: &Token) -> ConstantExpression<f32> {
        ConstantExpression::<f32> { value: match token.lexeme.parse::<f32>() {
            Ok(i) => i,
            Err(e) => panic!("Value is not valid for token type of NUMBER - {}", e),
        }}
    }
}