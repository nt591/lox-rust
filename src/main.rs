mod chunk;
mod disassembler;
mod value;
mod vm;

fn main() {
    let mut vm = vm::VM::new();
    let mut chunk = chunk::Chunk::new_chunk();
    
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
}
