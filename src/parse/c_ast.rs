#[derive(Debug, Clone)]
pub enum Expr {
    IntConstant{value: i32},
    Identifier{value: String},
}

impl Expr {
    pub fn new_int(value: &str) -> Expr {
        Expr::IntConstant {
            value: value.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Program{body: Box<Stmt>},
    FuncDef{name: Box<Expr>, body: Box<Stmt>},
    Return{value: Box<Expr>},
}