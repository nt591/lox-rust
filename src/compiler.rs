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

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next_rule(parse_rule: ParseRule) -> Precedence {
        match parse_rule.precedence {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => panic!("No matching rule higher than Primary"),
        }
    }
}

pub struct Compiler<'a> {
    scanner: Scanner,
    parser: Parser,
    compiling_chunk: &'a mut Chunk,
}

impl Compiler<'_> {
    pub fn new(chunk: &mut Chunk) -> Compiler {
        Compiler {
            scanner: Scanner::new(&"".to_string()),
            parser: Parser::new(),
            compiling_chunk: chunk,
        }
    }

    pub fn compile(&mut self,source: &String) -> bool {
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

    fn emit_bytes(&mut self, code1: OpCode, code2: OpCode) -> () {
        self.emit_byte(code1);
        self.emit_byte(code2);
    }

    //fn current_chunk(&self) -> Chunk {
      //  self.compiling_chunk
    //}

    fn end_compiler(&mut self) -> () {
        self.emit_return();
    }
    
    fn get_rule(&self, op_type: TokenType) -> ParseRule {
        RULES[op_type as usize] 
    }

    fn binary(&mut self) -> () {
        let op_type : TokenType = self.parser.previous.token_type;
        let rule : ParseRule = self.get_rule(op_type);
        self.parse_precedence(Precedence::next_rule(rule));
        match op_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            TokenType::BangEqual => self.emit_bytes(OpCode::Equal, OpCode::Not),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal),
            TokenType::Greater => self.emit_byte(OpCode::Greater),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::Less, OpCode::Not),
            TokenType::Less => self.emit_byte(OpCode::Less),
            TokenType::LessEqual => self.emit_bytes(OpCode::Greater, OpCode::Not),
            _ => (),
        }
    }

    fn literal(&mut self) -> () {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::False),
            TokenType::True => self.emit_byte(OpCode::True),
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            _ => (),
        }
    }

    fn number(&mut self) -> () {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        let val = Value::number_val(value);
        self.emit_constant(val);
    }

    fn string(&mut self) {
        let lexeme = self.parser.previous.lexeme.parse::<String>().unwrap();
        let len = lexeme.len();
        let value = &lexeme[1..len - 1];
        let val = Value::string_val(value.to_string());
        self.emit_constant(val);
    }

    fn grouping(&mut self) -> () {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) -> () {
        let last_seen_type = self.parser.previous.token_type;
        self.parse_precedence(Precedence::Unary);
        match last_seen_type {
            TokenType::Bang => self.emit_byte(OpCode::Not),
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => (),
        }
    }
    
    fn parse_precedence(&mut self, precedence: Precedence) -> () {
        self.advance();
        let prefix_rule : Option<ParserFunction> = self.get_rule(self.parser.previous.token_type).prefix;
        
        match prefix_rule {
            None => self.error("Expect expression."),
            Some(parse_fn) => parse_fn(self),
        }
        
        while precedence <= self.get_rule(self.parser.current.token_type).precedence {
            self.advance();
            let infix_rule : Option<ParserFunction> = self.get_rule(self.parser.previous.token_type).infix;
            match infix_rule {
                None => self.error("Unexpected call to infix rule"),
                Some(infix_fn) => infix_fn(self),
            }
        }
    }

    fn expression(&mut self) -> () {
        self.parse_precedence(Precedence::Assignment);    
    }

    fn emit_return(&mut self) -> () {
        self.emit_byte(OpCode::Return);
    }

    fn emit_constant(&mut self, value: Value) -> () {
        self.emit_byte(OpCode::Constant(value));
    }
}

/*
 * boilerplate setup for the parse precedence
 */

type ParserFunction = fn(&mut Compiler) -> ();

#[derive(Copy, Clone)]
struct ParseRule {
    infix: Option<ParserFunction>,
    prefix: Option<ParserFunction>,
    precedence: Precedence,
}

impl ParseRule {
    const fn infix(infix: ParserFunction, precedence: Precedence) -> ParseRule {
        ParseRule {
            infix: Some(infix),
            precedence,
            prefix: None,
        }
    }
    
    const fn prefix(prefix: ParserFunction, precedence: Precedence) -> ParseRule {
        ParseRule {
            infix: None,
            prefix: Some(prefix),
            precedence,
        }
    }

    const fn both(prefix: ParserFunction, infix: ParserFunction, precedence: Precedence) -> ParseRule {
        ParseRule {
            infix: Some(infix),
            prefix: Some(prefix),
            precedence,
        }
    }

    const fn neither() -> ParseRule {
        ParseRule {
            infix: None,
            prefix: None,
            precedence: Precedence::None,
        }
    }
}

// maps are heap allocated so this is faster
const RULES : [ParseRule; 40] = [
    ParseRule::prefix(|compiler| compiler.grouping(), Precedence::None), //left paren
    ParseRule::neither(), //right paren
    ParseRule::neither(), //left brace
    ParseRule::neither(), //right brace
    ParseRule::neither(), // comma
    ParseRule::neither(), // dot
    ParseRule::both(|compiler| compiler.unary(), |compiler| compiler.binary(), Precedence::Term), // minus
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Term), //plus
    ParseRule::neither(), //semicolon
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Factor), //slash
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Factor), //star
    ParseRule::prefix(|compiler| compiler.unary(), Precedence::None), // bang
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Equality), //bang equal
    ParseRule::neither(), //equal
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Equality), //equal equal
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Comparison), // greater
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Comparison), //greater equal
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Comparison), //less
    ParseRule::infix(|compiler| compiler.binary(), Precedence::Comparison), //less equal
    ParseRule::neither(), //identifier
    ParseRule::prefix(|compiler| compiler.string(), Precedence::None), //string
    ParseRule::prefix(|compiler| compiler.number(), Precedence::None), // number
    ParseRule::neither(), // and
    ParseRule::neither(), // class
    ParseRule::neither(), // else
    ParseRule::prefix(|compiler| compiler.literal(), Precedence::None), // false
    ParseRule::neither(), // for
    ParseRule::neither(), // fun
    ParseRule::neither(), // if
    ParseRule::prefix(|compiler| compiler.literal(), Precedence::None), // nil
    ParseRule::neither(), // or
    ParseRule::neither(), // print
    ParseRule::neither(), // return
    ParseRule::neither(), // super
    ParseRule::neither(), // this 
    ParseRule::prefix(|compiler| compiler.literal(), Precedence::None), // true
    ParseRule::neither(), // var
    ParseRule::neither(), // while
    ParseRule::neither(), // error 
    ParseRule::neither(), // eof
];

