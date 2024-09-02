
use crate::common::*;
use crate::value::*;

pub enum Opcode {
    OP_CONSTANT,
    // 后面补充
}

pub struct Chunk{
    count:i32,
    capacity:i32,
    code: String,
    lines: Vec<i32>,
    constants: ValueArray,
}

impl Chunk {
    pub fn initChunk(){
        todo!()
    }
    pub fn freeChunk(){
        todo!()
    }

    pub fn writeChunk() {
        todo!()
    }

    pub fn addConstant(){
        todo!()
    }
}
