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
