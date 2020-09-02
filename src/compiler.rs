use crate::scanner::{Scanner, TokenType, Token};
use crate::chunk::{Chunk, OpCode};
use crate::value::{Value, FunctionDef};

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

#[derive(Clone)]
struct Local {
    name: String,
    depth: usize,
    initialized: bool,
}

pub struct Compiler {
    functions: Vec<FunctionDef>,
    scanner: Scanner,
    parser: Parser,
    
    scope_depth: usize,
    locals: Vec<Local>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            scanner: Scanner::new(&"".to_string()),
            parser: Parser::new(),
            locals: Vec::new(),
            functions: Vec::new(),
            scope_depth: 0,
        }
    }
    
    fn current_function(&mut self) -> &mut FunctionDef {
        match self.functions.len() {
            0 => panic!("Compiler functions vector should never be empty"),
            len => &mut self.functions[len - 1]
        }
    }
    pub fn compile(&mut self,source: &String) -> Result<FunctionDef, &str> {
        self.scanner = Scanner::new(&source);
        self.reset_error_state();
        self.advance();
        
        while !self.match_token(TokenType::EOF) {
            self.declaration();
        }
        let errored = self.parser.had_error;
        let function = self.end_compiler();

        if errored {
            Err("Compiler Error")
        } else {
            Ok(function.clone())
        }
    }

    fn declaration(&mut self) -> () {
        if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }
        
        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    fn block(&mut self) -> () {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF) {
            // keep eating up every line up to semicolons as declarations
            self.declaration();
        }

        self.consume(TokenType::RightBrace, "Expect '}'  after block.");
    }

    fn statement(&mut self) -> () {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else if self.match_token(TokenType::For) {
            self.for_statement();
        } else if self.match_token(TokenType::If) {
           self.if_statement(); 
        } else if self.match_token(TokenType::While) {
            self.while_statement();    
        } else if self.match_token(TokenType::LeftBrace) {
            // open brace means new block scope
            self.begin_scope();
            self.block();
            self.end_scope();
        } else { 
            self.expression_statement();
        }
    }

    fn begin_scope(&mut self) -> () {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) -> () {
        self.scope_depth -= 1;

        // pop locals vector until we get rid of all of the old scope values
        // that is, anything with a value greater than current scope depth needs to die
        while self.locals.len() > 0 && self.locals.last().unwrap().depth > self.scope_depth {
            self.locals.pop();
            self.emit_byte(OpCode::Pop);
        }
    }

    fn print_statement(&mut self) -> () {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print);
    }

    fn for_statement(&mut self) -> () {
        // everything is locally scoped
        self.begin_scope();
        // we'll allow empty element in our clauses
        self.consume(TokenType::LeftParen, "Expect '( after 'for'." );
        if self.match_token(TokenType::Semicolon) {
            // no initializer
        } else if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            // need to make sure anything evaluated is discarded from the stack
            self.expression_statement();
        }

        let mut loop_start = self.current_chunk().code.len() - 1;

        // how do we know if we can exit the loop?
        // first we see if the next char is a semicolon
        // if it is, we have something like for (x = 1;;)
        // if it ISNT, there's a body inside like for (x = 1; x < 10;;)

        let mut exit_jump: Option<usize> = None;
        if !self.match_token(TokenType::Semicolon) {
            self.expression();
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition");
            

           exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse(0)));
           self.emit_byte(OpCode::Pop); // pop condition
        }
        
    
        // if we haven't yet seen a right paren, the next thing must be the incrementor
        if !self.match_token(TokenType::RightParen) {
            // jump over the incrementor the first time, execute the body
            // then jump BACK to the incrementor and reevaluate the condition
            // http://www.craftinginterpreters.com/image/jumping-back-and-forth/for.png
            let body_jump = self.emit_jump(OpCode::Jump(0));
            let increment_start = self.current_chunk().code.len() - 1;
        
            self.expression();
            self.emit_byte(OpCode::Pop);
            self.consume(TokenType::RightParen, "Expect ')' after for clauses.");
            self.emit_loop(loop_start);
            loop_start = increment_start;

            self.patch_jump(body_jump);
        }
        self.statement();
        self.emit_loop(loop_start);
        
        // we jump to here in case we've finished the condition for looping
        if let Some(jump) = exit_jump {
            self.patch_jump(jump);
            self.emit_byte(OpCode::Pop); // get the condition off stack
        }
        self.end_scope();
    }

    fn while_statement(&mut self) -> () {
        // this tells our loop where to run back to re-evaluate expression
        let loop_start = self.current_chunk().code.len() - 1;
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        // add a jump instruction to the end if while loop condition is false
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        // if not false, clean up the jump
        self.emit_byte(OpCode::Pop);
        self.statement();

        // since we didn't exit we need to go back up and figure out how to re-run the statement
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::Pop);
    }

    fn if_statement(&mut self) -> () {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        let then_jump_patch_idx = self.emit_jump(OpCode::JumpIfFalse(0)); // emit placeholder
        self.emit_byte(OpCode::Pop);
        self.statement();

        // need the location of the end of the if consequence in order to else-patch
        let else_jump_patch_idx = self.emit_jump(OpCode::Jump(0));
        self.patch_jump(then_jump_patch_idx);
        self.emit_byte(OpCode::Pop);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump_patch_idx);
    }

    fn emit_jump(&mut self, jump_op: OpCode) -> usize {
       self.emit_byte(jump_op);
       self.current_chunk().code.len() - 1
    }

    fn emit_loop(&mut self, loop_start: usize) -> () {
        let loop_offset = self.current_chunk().code.len() - loop_start;
        
        self.emit_byte(OpCode::Loop(loop_offset));
    }

    fn patch_jump(&mut self, offset: usize) -> () {
        let jump_length = self.current_chunk().code.len() - offset - 1;
        let new_op = match self.current_chunk().code[offset].code {
            OpCode::JumpIfFalse(0) => OpCode::JumpIfFalse(jump_length),
            OpCode::Jump(0) => OpCode::Jump(jump_length),
            _ => panic!("offset for patch_jump points to invalid instruction"),
        };

        self.current_chunk().code[offset].code = new_op;
    }

    fn var_declaration(&mut self) -> () {
        let global = self.parse_variable("Expect variable name.");

        if self.match_token(TokenType::Equal) {
            self.expression()
        } else {
            self.emit_byte(OpCode::Nil);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration");

        self.define_variable(global);
    }

    fn expression_statement(&mut self) -> () {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::Pop);
    }

    fn reset_error_state(&mut self) -> () {
        self.parser.panic_mode = false;
        self.parser.had_error = false;
    }

    fn parse_variable(&mut self, msg: &str) -> String {
        self.consume(TokenType::Identifier, msg);

        self.declare_variable();
        if self.scope_depth > 0 {
            // todo - stop using this dummy value
            "".to_string()
        } else {
            self.identifier_constant(&self.parser.previous)
        }
    }

    fn define_variable(&mut self, global: String) -> () {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return ();
        }
        self.emit_byte(OpCode::DefineGlobal(global));
    }

    fn mark_initialized(&mut self) -> () {
        let len = self.locals.len();
        self.locals[len - 1].initialized = true;
    }

    fn declare_variable(&mut self) -> () {
        if self.scope_depth == 0 {
            return;
        }

        let token = self.parser.previous.clone();
        let locals = self.locals.clone();

        for local in locals.iter().rev() {
            if local.initialized && local.depth < self.scope_depth {
                break;
            } 

            if local.name == token.lexeme {
                self.error("Variable with this name already declared in this scope.");
            }
        }

        self.add_local(token);
    }

    fn add_local(&mut self, token: Token) -> () {
        let local = Local {
            name: token.lexeme.clone(),
            depth: self.scope_depth,
            initialized: false,
        };

        self.locals.push(local);
    }

    fn identifier_constant(&self, token: &Token) -> String {
        token.lexeme.clone()
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

    fn synchronize(&mut self) -> () {
        self.parser.panic_mode = false;

        while self.parser.current.token_type != TokenType::EOF {
            if self.parser.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.parser.current.token_type {
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return => return (),
                _ => (),
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> () {
        if self.parser.current.token_type == token_type {
            self.advance();
        } else {
            self.error_at_current(&msg);
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            false
        } else {
            self.advance();
            true
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.parser.current.token_type == token_type
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
        let line = self.parser.previous.line;
        self.current_chunk().write(code, line);
    }

    fn emit_bytes(&mut self, code1: OpCode, code2: OpCode) -> () {
        self.emit_byte(code1);
        self.emit_byte(code2);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        match self.functions.len() {
            0 => panic!("compiler should never have an empty functions vector"),
            len => &mut self.functions[len-1].chunk
        }
    }

    fn end_compiler(&mut self) -> &mut FunctionDef {
        self.emit_return();
        self.current_function()
    }
    
    fn get_rule(&self, op_type: TokenType) -> ParseRule {
        RULES[op_type as usize] 
    }

    fn binary(&mut self, _can_assign: bool) -> () {
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

    fn literal(&mut self, _can_assign: bool) -> () {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::False),
            TokenType::True => self.emit_byte(OpCode::True),
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            _ => (),
        }
    }

    fn number(&mut self, _can_assign: bool) -> () {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        let val = Value::number_val(value);
        self.emit_constant(val);
    }

    fn string(&mut self, _can_assign: bool) {
        let lexeme = self.parser.previous.lexeme.parse::<String>().unwrap();
        let len = lexeme.len();
        let value = &lexeme[1..len - 1];
        let val = Value::string_val(value.to_string());
        self.emit_constant(val);
    }

    fn variable(&mut self, can_assign: bool) -> () {
        self.named_variable(self.parser.previous.clone(), can_assign);
    }

    fn named_variable(&mut self, token: Token, can_assign: bool) -> () {
        let get_op;
        let set_op;
        
        if let Some(arg) = self.resolve_local(&token) {
            set_op = OpCode::SetLocal(arg);
            get_op = OpCode::GetLocal(arg);
        } else {
            let arg = self.identifier_constant(&token);
            set_op = OpCode::SetGlobal(arg.clone());
            get_op = OpCode::GetGlobal(arg.clone());
        }
        // we'll check for setters vs getters
        if can_assign && self.match_token(TokenType::Equal) {
            self.expression();
            self.emit_byte(set_op);
        } else {
            self.emit_byte(get_op);
        }
    }

    fn resolve_local(&mut self, token: &Token) -> Option<usize> {
        let locals = self.locals.clone();
        for (idx, local) in locals.iter().enumerate().rev() {
            if local.name == token.lexeme {
                if !local.initialized {
                    self.error("Cannot read local variable in its own initializer.");
                }
                return Some(idx);
            } 
        }

        None
    }

    fn grouping(&mut self, _can_assign: bool) -> () {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self, _can_assign: bool) -> () {
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
        let can_assign = precedence <= Precedence::Assignment;

        match prefix_rule {
            None => self.error("Expect expression."),
            Some(parse_fn) => parse_fn(self, can_assign),
        }
        
        while precedence <= self.get_rule(self.parser.current.token_type).precedence {
            self.advance();
            let infix_rule : Option<ParserFunction> = self.get_rule(self.parser.previous.token_type).infix;
            match infix_rule {
                None => self.error("Unexpected call to infix rule"),
                Some(infix_fn) => infix_fn(self, can_assign),
            }
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.error("Invalid assignment target.");
        }
    }

    fn and(&mut self, _can_assign: bool) -> () {
        let end_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);
        self.parse_precedence(Precedence::And);

        self.patch_jump(end_jump);
    }

    fn or(&mut self, _can_assign: bool) -> () {
        // if the lefthand side is falsey, we need to parse and evaluate the righthand side
        // so else jump says "hey, if the top of the stack is false basically just keep going"
        // if the lefthand side is truthy, tthen it'll run into the end_jump jump which skips parsing the righthand side

        let else_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        let end_jump = self.emit_jump(OpCode::Jump(0));

        self.patch_jump(else_jump);
        self.emit_byte(OpCode::Pop);

        self.parse_precedence(Precedence::Or);

        self.patch_jump(end_jump);
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

type ParserFunction = fn(&mut Compiler, bool) -> ();

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
    ParseRule::prefix(|compiler, can_assign| compiler.grouping(can_assign), Precedence::None), //left paren
    ParseRule::neither(), //right paren
    ParseRule::neither(), //left brace
    ParseRule::neither(), //right brace
    ParseRule::neither(), // comma
    ParseRule::neither(), // dot
    ParseRule::both(|compiler, can_assign| compiler.unary(can_assign), |compiler, can_assign| compiler.binary(can_assign), Precedence::Term), // minus
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Term), //plus
    ParseRule::neither(), //semicolon
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Factor), //slash
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Factor), //star
    ParseRule::prefix(|compiler, can_assign| compiler.unary(can_assign), Precedence::None), // bang
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Equality), //bang equal
    ParseRule::neither(), //equal
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Equality), //equal equal
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), // greater
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), //greater equal
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), //less
    ParseRule::infix(|compiler, can_assign| compiler.binary(can_assign), Precedence::Comparison), //less equal
    ParseRule::prefix(|compiler, can_assign| compiler.variable(can_assign), Precedence::None), //identifier
    ParseRule::prefix(|compiler, can_assign| compiler.string(can_assign), Precedence::None), //string
    ParseRule::prefix(|compiler, can_assign| compiler.number(can_assign), Precedence::None), // number
    ParseRule::infix(|compiler, _can_assign| compiler.and(_can_assign), Precedence::And), // and
    ParseRule::neither(), // class
    ParseRule::neither(), // else
    ParseRule::prefix(|compiler, can_assign| compiler.literal(can_assign), Precedence::None), // false
    ParseRule::neither(), // for
    ParseRule::neither(), // fun
    ParseRule::neither(), // if
    ParseRule::prefix(|compiler, can_assign| compiler.literal(can_assign), Precedence::None), // nil
    ParseRule::infix(|compiler, _can_assign| compiler.or(_can_assign), Precedence::Or), // or
    ParseRule::neither(), // print
    ParseRule::neither(), // return
    ParseRule::neither(), // super
    ParseRule::neither(), // this 
    ParseRule::prefix(|compiler, can_assign| compiler.literal(can_assign), Precedence::None), // true
    ParseRule::neither(), // var
    ParseRule::neither(), // while
    ParseRule::neither(), // error 
    ParseRule::neither(), // eof
];

