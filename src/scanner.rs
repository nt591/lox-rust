pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars = source.chars().collect();
        Scanner {
            source: chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        self.error_token("Unexpected character.".to_string())
    }

    pub fn is_at_end(&self) -> bool {
        self.source[self.current] == '\0'
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            lexeme: self.source[self.start..self.current].iter().collect(),
        }
    }

    fn error_token(&self, message: String) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
    pub lexeme: String
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // single characters
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // one or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    // Literals
    Identifier, TokenString, Number,

    // keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, 
    EOF,
}

