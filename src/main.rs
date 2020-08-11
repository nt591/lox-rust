mod chunk;
mod disassembler;

fn main() {
    println!("{}", 0);
    println!("{:?}", chunk::OpCode::Return);
    let mut chunk = chunk::Chunk::new_chunk();
    chunk.write_chunk(chunk::OpCode::Return);
    let name = String::from("test chunk");
    disassembler::disassemble_chunk(chunk, name);
    println!("done!");
}
