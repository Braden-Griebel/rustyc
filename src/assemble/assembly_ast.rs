
/// Assembly Instructions AST
#[derive(Debug, Clone)]
pub enum Instr {
    /// Represents an assembly program
    Program{body: Box<Instr>},
    /// Represents a function definition
    FuncDef{name: Box<Instr>, instructions: Vec<Box<Instr>>},
    /// Represents a Move Operation
    Mov{src: Box<Instr>, dst: Box<Instr>},
    /// Represents a return instruction
    Ret,
    /// Represents an immediate value
    Imm{value: i32},
    /// Represents a single register
    Register, // Currently just eax
    /// Represents an identifier
    Identifier{value: String},
    /// Represents a series of instructions
    Series{instructions: Vec<Box<Instr>>}
}