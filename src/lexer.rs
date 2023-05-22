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

    pub fn new(source: &str) -> Lexer {
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
                break;
            }
            self.start = self.current;
            let c = self.source[self.start];
            match c {
                ' ' => {
                    self.forward();
                    continue;
                },
                '.' => {
                    if !Self::is_numeric(self.peek()) {
                        tokens.push(Token { token_type: TokenType::DOT, lexeme: '.'.to_string(), line: self.line});
                        self.forward();
                    } else {
                        tokens.push(self.get_number());
                    }
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
                '"' => {
                    tokens.push(self.get_string());
                }
                _ => {
                    if Self::is_numeric(c) || c == '-'{
                        tokens.push(self.get_number());
                    } else if Self::is_alphanumeric(c) {
                        tokens.push(self.get_identifier());
                    }
                }
            }
        }

        tokens.push(Token { token_type: TokenType::EOF, lexeme: '\0'.to_string(), line: self.line});

        tokens
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_numeric(c: char) -> bool {
        c.is_digit(10) || c == '.'
    }

    fn get_identifier(&mut self) -> Token {
        let condition = |s: &mut Lexer| -> bool { Self::is_alphanumeric(s.source[s.current]) };
        let raw = self.collect_until(condition, None);

        Token {
            lexeme: raw,
            line: self.line,
            token_type: TokenType::IDENTIFIER
        }
    }

    fn get_number(&mut self) -> Token {
        if self.source[self.current] == '-' {
            self.forward();
        }
        let condition = |s: &mut Lexer| -> bool { Self::is_numeric(s.source[s.current]) };
        let raw = self.collect_until(condition, None);

        if raw.matches('.').count() > 1 {
            panic!("number has more than one '.'");
        }

        Token {
            lexeme: raw,
            line: self.line,
            token_type: TokenType::NUMBER
        }
    }

    fn get_string(&mut self) -> Token {
        // skip opening quotes
        self.forward();
        self.start += 1;

        let condition = |s: &mut Lexer| -> bool { s.source[s.current] != '"' };
        let raw = self.collect_until(condition, Some(format!("string on line {} is not closed", self.line)));

        // skip over closing quotes
        self.forward();

        Token {
            lexeme: raw,
            line: self.line,
            token_type: TokenType::STRING
        }

    }

    fn collect_until(&mut self, condition: impl Fn(&mut Lexer) -> bool, eof_reached_error: Option<String>) -> String {
        while condition(self) {
            self.forward();
            if self.current >= self.source.len() {
                match eof_reached_error {
                    Some(e) => panic!("{}", e),
                    _ => break
                }
            }
        }

        if self.start == self.current {
            self.source[self.current].to_string()
        } else {
            String::from_iter(self.source[self.start..self.current].iter())
        }
    }

    fn peek(&mut self) -> char {
        self.source[self.current + 1]
    }

    fn forward(&mut self) {
        self.current += 1
    }

}