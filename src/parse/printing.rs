use super::ast::{Stmt, Expr};


pub struct Printer {
    current_level: u32,
    output: String,
}

impl Printer {
    pub fn new() -> Printer {
        Printer { current_level: 0, output: String::new() }
    }
    
    pub fn print_stmt(&mut self, stmt: &Stmt) {
        self.clear();
        self.stmt_to_string(stmt);
        print!("{}", &self.output);
    }
    
    #[allow(dead_code)]
    pub fn print_expr(&mut self, expr: &Expr) {
        self.current_level =0;
        self.output = String::new();
        self.expr_to_string(expr);
        print!("{}", &self.output);
    }
    
    fn stmt_to_string(&mut self, stmt: &Stmt) {
        self.get_to_level();
        match stmt {
            Stmt::Program { body } => {
                self.output.push_str("Program(\n");
                self.current_level +=1;
                self.stmt_to_string(body);
                self.current_level-=1;
                self.output.push_str(")");
            }
            Stmt::FuncDef { name, body } => {
                self.output.push_str("Function(\n");
                self.current_level +=1;
                self.get_to_level();
                self.output.push_str("name=");
                self.expr_to_string(name);
                self.output.push_str(",\n");
                self.get_to_level();
                self.output.push_str("body={\n");
                self.current_level+=1;
                self.stmt_to_string(body);
                self.current_level-=1;
                self.get_to_level();
                self.output.push_str("}\n");
                self.current_level-=1;
                self.get_to_level();
                self.output.push_str(")\n");
            }
            Stmt::Return { value } => {
                self.output.push_str("Return(\n");
                self.current_level += 1;
                self.get_to_level();
                self.expr_to_string(value);
                self.output.push_str("\n");
                self.current_level-=1;
                self.get_to_level();
                self.output.push_str(")\n");
            }
        }
    }
    
    fn expr_to_string(&mut self, expr: &Expr) {
        match expr {
            Expr::IntConstant { value } => {
                self.output.push_str("Constant(");
                self.output.push_str(&value.to_string());
                self.output.push(')');
            }
            Expr::Identifier { value } => {
                self.output.push('"');
                self.output.push_str(&value.to_string());
                self.output.push('"');
            }
        }
    }
    
    fn get_to_level(&mut self){
        for _ in 0..self.current_level {
            self.output.push_str("    ");
        }
    }
    
    fn clear(&mut self){
        self.output = String::new();
        self.current_level = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex;
    use crate::parse;
    #[test]
    fn test_expr_to_string() {
        // Create Printer object
        let mut printer = Printer::new();
        // Test Constant Int Printer
        printer.expr_to_string(&Expr::IntConstant { value: 1 });
        assert_eq!(printer.output, "Constant(1)");
        printer.clear();
        // Test Identifier Printer
        printer.expr_to_string(&Expr::Identifier { value: "main".to_string() });
        assert_eq!(printer.output, "\"main\"");
        printer.clear();
    }
    
    #[test]
    fn test_stmt_to_string() {
        let mut lexer = lex::Lexer::new(String::from("int main(void){return 2;}"));
        let tokens = lexer.tokenize().unwrap();
        let mut parser = parse::parsing::Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut printer = Printer::new();
        printer.stmt_to_string(&ast);
        let result = printer.output.clone();
        printer.clear();
        assert_eq!(result, 
"Program(
    Function(
        name=\"main\",
        body={
            Return(
                Constant(2)
            )
        }
    )
)");
        
    }
}