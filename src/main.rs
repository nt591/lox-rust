use std::env;
use std::io::Read;

mod compiler;
mod chunk;
mod disassembler;
mod scanner;
mod value;
mod vm;

fn main() {
    let mut vm = vm::VM::new();
    /*
     * let mut chunk = chunk::Chunk::new_chunk();
    
    chunk.write_chunk(chunk::OpCode::Constant(1.2), 123);
    chunk.write_chunk(chunk::OpCode::Constant(3.4), 123);
    chunk.write_chunk(chunk::OpCode::Add, 123);

    chunk.write_chunk(chunk::OpCode::Constant(5.6), 123);
    chunk.write_chunk(chunk::OpCode::Divide, 123);
    chunk.write_chunk(chunk::OpCode::Negate,123);
    
    chunk.write_chunk(chunk::OpCode::Return, 123);

    let name = String::from("test chunk");
    disassembler::disassemble_chunk(chunk.clone(), name);
    vm.interpret(chunk);
    println!("done!");
    */

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => match run_file(&args[1]) {
            Err(_) => {
                eprintln!("Invalid file at {}", args[1]);
                std::process::exit(74);
            },
            _ => (),
        },
        _ => {
            eprintln!("Usage: clox [path]");
            std::process::exit(64);
        }
    }
}

fn interpret(source: String) -> vm::InterpretResult {
    compiler::compile(source);
    vm::InterpretResult::Ok
}

fn repl() -> () {
    loop {
       let mut input = String::new();

        print!("> ");
        match std::io::stdin().read_to_string(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!();
                break;
            }
        }

        interpret(input.clone());
    }
}

fn run_file(path: &String) -> std::io::Result<()> {
    let mut buffer = String::new();
    let mut f = std::fs::File::open(path)?;
    f.read_to_string(&mut buffer)?;
    let result : vm::InterpretResult = interpret(buffer);

    match result {
        vm::InterpretResult::CompileError => std::process::exit(65),
        vm::InterpretResult::RuntimeError => std::process::exit(70),
        _ => Ok(()),
    }
}
