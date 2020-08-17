pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        let chars = source.chars().collect();
        Scanner {
            source: chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c : char = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_token('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_token('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_token('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_token('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            },
            '"' => self.string(),
            ch if self.is_digit(ch) => self.number(),
            ch if self.is_alpha(ch) => self.identifier(),
            _ => self.error_token("Unexpected character.".to_string())
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source[self.current] == '\0'
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            lexeme: self.source[self.start..self.current].iter().collect(),
            start: self.start,
        }
    }

    fn error_token(&self, message: String) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line,
            start: self.start,
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.".to_string())
        } else {
            self.advance();
            self.make_token(TokenType::TokenString)
        }
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn number(&mut self) -> Token {
        // capture all the digits we see
        // if the thing right after is a period, and after that is more digits
        // assume we're looking at a float, so capture whole thing
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // we're in the middle of a float, looking at decimal now
            // consume decimal, and capture rest of num
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn is_alpha(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') ||
        (ch >= 'A' && ch <= 'A') ||
        ch == '_'
    }
    
    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            },
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            },
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        let start_idx : usize = self.start + start;
        let end_idx : usize = start_idx + length + 1;
        let substr : String = self.source[start_idx..end_idx].into_iter().collect();
        
        if (self.current - self.start == start + length) 
            && substr == rest {
            token_type    
        } else {
            TokenType::Identifier
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() { 
            false 
        } else if self.source[self.current] != expected { 
           false 
        } else {
            self.current += 1;
            true
        }
    }



    fn skip_whitespace(&mut self) -> () {
        loop {
            let c: char = self.peek();

            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    break;
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }    
                    }
                    break ();
                }
                _ => break (),
            }
        }
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
    pub start: usize,
    pub lexeme: String
}

#[derive(PartialEq, Debug, Clone, Copy)]
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

    // starting place 
    _Default,
}

