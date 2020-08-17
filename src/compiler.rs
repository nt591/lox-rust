use crate::scanner::{Scanner, TokenType, Token};
use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { 
            current: Token { 
                token_type: TokenType::_Default,
                line: 0,
                lexeme: "".to_string(),
                start: 0,
            },
            previous: Token {
                token_type: TokenType:: _Default,
                line: 0,
                lexeme: "".to_string(),
                start: 0,
            },
            had_error: false,
            panic_mode: false,
        }
    }
}

pub struct Compiler {
    scanner: Scanner,
    chunk: Chunk,
    parser: Parser,
    compiling_chunk: Chunk,
}

impl Compiler {

    pub fn new() -> Compiler {
        Compiler {
            scanner: Scanner::new(&"".to_string()),
            chunk: Chunk::new_chunk(),
            parser: Parser::new(),
            compiling_chunk: Chunk::new_chunk()
        }
    }

    pub fn compile(&mut self,source: &String, chunk: &Chunk) -> bool {
        self.scanner = Scanner::new(&source);
        self.reset_error_state();
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.");
        self.end_compiler();
        !self.parser.had_error
    }

    fn reset_error_state(&mut self) -> () {
        self.parser.panic_mode = false;
        self.parser.had_error = false;
    }

    fn advance(&mut self) -> () {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break ()
            }
            self.error_at_current(&self.parser.current.lexeme.clone());
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> () {
        if self.parser.current.token_type == token_type {
            self.advance();
        } else {
            self.error_at_current(&msg);
        }
    }

    fn error_at_current(&mut self, msg: &str) -> () {
        self.error_at(self.parser.current.clone(), &msg)
    }

    fn error(&mut self, msg: &str) -> () {
        self.error_at(self.parser.previous.clone(), &msg)
    }

    fn error_at(&mut self, token: Token, msg: &str) -> () {
        if self.parser.panic_mode { return }; 
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);
        match token.token_type {
            TokenType::EOF => eprint!(" at end"),
            _ => (),

        }

        eprintln!(": {}", msg);
        self.parser.had_error = true;
    }

    fn emit_byte(&mut self, code: OpCode) -> () {
        self.compiling_chunk.write(code, self.parser.previous.line);
    }

    //fn current_chunk(&self) -> Chunk {
      //  self.compiling_chunk
    //}

    fn end_compiler(&mut self) -> () {
        self.emit_return();
    }

    fn number(&mut self) -> () {
        let value = self.parser.previous.lexeme.parse::<Value>().unwrap();
        self.emit_constant(value);
    }

    fn grouping(&mut self) -> () {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) -> () {
        let last_seen_type = self.parser.previous.token_type;
        self.expression();
        match last_seen_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => (),
        }
    }

    fn expression(&self) -> () {
        
    }

    fn emit_return(&mut self) -> () {
        self.emit_byte(OpCode::Return);
    }

    fn emit_constant(&mut self, value: Value) -> () {
        self.emit_byte(OpCode::Constant(value));
    }
}

