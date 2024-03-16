use std::iter::Map;

use crate::parser::Opcode;

pub struct Program {
    globals: Map<String, Variable>,
    functions: Map<String, Function>,
}

pub enum Variable {
    Atomic {
        name: String,
        initial: Option<Value>
    },
    Array {
        name: String,
        size: u32,
        initial: Vec<Value>
    },
}

pub enum Value {
    Constant(i32),
    Variable(String),
}

pub struct Function {
    name: String,
    arguments: Vec<String>,
    // @todo parse local var and extern names ?
    statements: Vec<Opcode>,
}
