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
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(&mut vm),
        2 => match run_file(&args[1], &mut vm) {
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

fn repl(machine: &mut vm::VM) -> () {
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

        machine.interpret(&input);
    }
}

fn run_file(path: &String, machine: &mut vm::VM) -> std::io::Result<()> {
    let mut buffer = String::new();
    let mut f = std::fs::File::open(path)?;
    f.read_to_string(&mut buffer)?;
    let result : vm::InterpretResult = machine.interpret(&buffer);

    match result {
        vm::InterpretResult::CompileError => std::process::exit(65),
        vm::InterpretResult::RuntimeError => std::process::exit(70),
        _ => Ok(()),
    }
}
