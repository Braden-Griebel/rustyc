use crate::parse::c_ast::{Stmt,Expr};
use crate::assemble::assembly_ast::Instr;

// This is currently just basically acting as a namespace, but I want the possibility of 
// carried state in the future without significant rewrites
pub struct Assembler{
}

impl Assembler{
    fn new(c_ast: Stmt) -> Assembler{
        Assembler{}
    }
    
    fn assemble(&self, stmt: Stmt) -> Result<Box<Instr>,AssemblerError>{
        self.assemble_stmt(stmt)
    }
    
    fn assemble_stmt(&self, stmt: Stmt) -> Result<Box<Instr>, AssemblerError>{
        match stmt{
            Stmt::Program { body } => {
                let body = self.assemble_stmt(*body)?;
                Ok(Box::new(Instr::Program {body}))
            }
            Stmt::FuncDef { name, body } => {
                let name = match *name{
                    Expr::Identifier { value } => {value.clone()},
                    _=>{return Err(AssemblerError::InvalidFuncName)}
                };
                let name_identifier = Box::new(Instr::Identifier { value:name });
                let mut func_body:Vec<Box<Instr>> = Vec::new();
                // Figure out return value
                let ret_val = match *body {
                    Stmt::Return { value } => {
                        self.assemble_expr(*value)?
                    }
                    _=>{return Err(AssemblerError::InvalidReturn)}
                };
                // Add return value to the func body
                func_body.push(Box::new(Instr::Mov{src:ret_val, dst: Box::new(Instr::Register)}));
                // Add the return instruction as the final instruction in the 
                func_body.push(Box::new(Instr::Ret));
                Ok(Box::new(Instr::FuncDef {name:name_identifier, instructions:func_body}))
            }
            Stmt::Return { value } => {
                match *value{
                    Expr::IntConstant { value } => {
                        let mut series:Vec<Box<Instr>> = Vec::new();
                        let ret_val = Box::new(Instr::Imm{value });
                        // Move the return value to eax register
                        series.push(Box::new(Instr::Mov{src: ret_val, dst: Box::new(Instr::Register)}));
                        // Return from the function
                        series.push(Box::new(Instr::Ret));
                        Ok(Box::new(Instr::Series {instructions: series}))
                        
                    }
                    _ => {Err(AssemblerError::InvalidReturn)}
                }
            }
        }
    }
    
    fn assemble_expr(&self, expr: Expr) -> Result<Box<Instr>, AssemblerError>{
        match expr{
            Expr::IntConstant { value } => {
                Ok(Box::new(Instr::Imm { value: value.clone() }))
            }
            Expr::Identifier { value } => {
                Ok(Box::new(Instr::Identifier {value: value.clone()}))
            }
        }
    }
}

pub enum AssemblerError{
    InvalidFuncName,
    InvalidFuncBody,
    InvalidReturn
}