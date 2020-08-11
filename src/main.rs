mod chunk;
mod disassembler;
mod value;

fn main() {
    let mut chunk = chunk::Chunk::new_chunk();
    
    chunk.write_chunk(chunk::OpCode::Constant(1.2));
    chunk.write_chunk(chunk::OpCode::Return);

    let name = String::from("test chunk");
    disassembler::disassemble_chunk(chunk, name);
    println!("done!");
}
