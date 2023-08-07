use std::collections::HashMap;
pub type Env = HashMap<String, i32>;
pub type FunctionTable = HashMap<String, Declaration>;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAR,
    RPAR,
    LBRACE,
    RBRACE,
    COMMA,
    EQ,
    NUMBER(i32),
    IF,
    ELSE,
    IDENT(String),
    SEMICOLON,
    FN, 
    RETURN,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Function {
        params: Vec<String>,
        body: Box<Statement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Number(i32),
    Var(String),
    FunctionCall {
        id: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    ExprStatement { 
        expr: Expr 
    },
    CompoundStatement {
        st1: Box<Statement>,
        st2: Box<Statement>,
    },
    Print {
        // TODO 関数作ったら関数にする
        expr: Expr,
    },
    FunctionDefine {
        id: String,
        params: Vec<String>,
        body: Box<Statement>,
    },
    FunctionCall {
        expr: Expr,
    },
    Return {
        expr: Box<Expr>,
    },
    Assign {
        id: String,
        e: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then: Box<Statement>,
        els: Box<Statement>,
    },
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Syntax {
    Statement(Statement),
}
