pub struct Lexer {
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>,
    source: Vec<char>
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

#[derive(Debug)]
pub enum TokenType {
    // Punctuation
    DOT,

    // Operators
    ASSIGN,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // MISC
    EOL,
    EOF
}

impl Lexer {

    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new()
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        for i in 0..self.source.len() {
            if self.current == self.source.len() {
                tokens.push(Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: self.line});
                break;
            }
            self.start = self.current;
            let c = self.source[self.start];
            match c {
                ' ' => {
                    self.forward();
                    continue;
                },
                '\n' => {
                    tokens.push(Token { token_type: TokenType::EOL, lexeme: '\t'.to_string(), line: self.line});
                    self.line += 1;
                    self.forward();
                },
                '=' => {
                    tokens.push(Token { token_type: TokenType::ASSIGN, lexeme: '='.to_string(), line: self.line});
                    self.forward();
                },
                _ => tokens.push(self.get_identifier())
            }
        }

        tokens
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn get_identifier(&mut self) -> Token {
        while Self::is_alphanumeric(self.source[self.current]) {
            self.forward();
            if self.current >= self.source.len() {
                break;
            }
        }
        let raw = if self.start == self.current {
            self.source[self.current].to_string()
        } else {
            String::from_iter(self.source[self.start..self.current].iter())
        };

        Token {
            lexeme: raw,
            line: self.line,
            token_type: TokenType::IDENTIFIER
        }
    }

    fn forward(&mut self) {
        self.current += 1
    }

}