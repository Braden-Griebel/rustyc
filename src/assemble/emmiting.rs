use std::path::PathBuf;
use super::assembly_ast::Instr;

pub struct Emitter{
    output: String
}

impl Emitter{
    pub fn new() -> Emitter{
        Emitter{output: String::new()}
    }

    pub fn emit(&mut self, file: PathBuf, instr: Box<Instr>)->Result<(), EmitError>{
        let assembly = self.emit_str(instr)?;
        match std::fs::write(file, assembly){
            Ok(_) => {}
            Err(_) => {return Err(EmitError::FileError)}
        };
        Ok(())
    }

    fn emit_str(&mut self, instr: Box<Instr>)->Result<String, EmitError>{
        self.emit_instr(instr)?;
        Ok(self.output.clone())
    }

    fn emit_instr(&mut self, instr:Box<Instr>)->Result<(), EmitError>{
        match *instr{
            Instr::Program { body } => {
                self.output.push_str("    .globl main\n");
                self.emit_instr(body)?;
                self.output.push_str("    .section .note.GNU-stack,\"\",@progbits");
                Ok(())
            }
            Instr::FuncDef { name, instructions } => {
                let func_name = match *name {
                    Instr::Identifier { value } => {
                        value.clone()
                    }
                    _=>{return Err(EmitError::InvalidFunctionName)}
                };
                self.output.push_str(format!("{}:\n", func_name).as_str());
                for instr in instructions{
                    self.emit_instr(instr)?;
                }
                Ok(())
            }
            Instr::Mov { src, dst } => {
                self.output.push_str("    movl");
                self.output.push_str("    ");
                self.emit_instr(src)?;
                self.output.push_str(", ");
                self.emit_instr(dst)?;
                self.output.push_str("\n");
                Ok(())
            }
            Instr::Ret => {
                self.output.push_str("    ret\n");
                Ok(())
            }
            Instr::Imm { value } => {
                self.output.push_str(format!("${}", value).as_str());
                Ok(())
            }
            Instr::Register => {
                self.output.push_str("%eax");
                Ok(())
            }
            Instr::Identifier { value } => {
                self.output.push_str(value.as_str());
                Ok(())
            }
            Instr::Series { instructions } => {
                for instr in instructions{
                    self.emit_instr(instr)?;
                };
                Ok(())
            }
        }
    }
}


pub enum EmitError{
    FileError,
    InvalidFunctionName,
}