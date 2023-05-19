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
            self.start = self.current;
            let c = self.source[i];
            match c {
                _ => {
                    if Self::is_alphanumeric(c) {
                        tokens.push(self.get_identifier());
                    }
                }
            }
        }

        tokens
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn get_identifier(&mut self) -> Token {
        while Self::is_alphanumeric(self.peek()) {
            self.forward();
        }
        let raw = self.source[self.start..self.current].into_iter().collect::<String>();
        println!("{raw}");
        Token {
            lexeme: raw,
            line: self.line,
            token_type: TokenType::IDENTIFIER
        }

    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        };
        self.source[self.current + 1]
    }

    fn forward(&mut self) {
        self.current += 1;
    }

}