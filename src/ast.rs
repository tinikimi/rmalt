use std::sync::Arc;
use value::Value;
use value::_Str;
use core::interpreter::InterpreterContext;

pub struct QuoteAst {
    pub expr: Ast,
}

pub struct TupleAst {
    pub tuple: Vec<Ast>,
}

pub struct CondAst {
    pub pair: Vec<(Ast, Ast)>,
}

pub struct MatchAst {
    pub cval: Ast, // cond value
    pub pair: Vec<(Ast, Ast)>,
}

pub struct LoopAst {
    pub expr: Vec<Ast>,
}

pub struct ForAst {
    pub name: String,
    pub tuple: Ast, // cond value
    pub expr: Vec<Ast>,
}

pub struct WhileAst {
    pub cond: Ast,
    pub expr: Vec<Ast>,
}

pub struct DefunAst {
    pub name: String,
    pub args: Vec<String>,
    pub expr: Vec<Ast>,
}

pub struct FCallAst {
    pub list: Vec<Ast>,
}

pub enum Ast {
    ///## 字面量
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    UInt(u64),
    Float(f64),

    String(_Str),
    Symbol(_Str),

    ///### 元组
    Tuple(Arc<TupleAst>),

    ///### 引用
    Quote(Arc<QuoteAst>),

    ///## 控制结构
    ///### 分支结构
    Cond(Arc<CondAst>),
    Match(Arc<MatchAst>),

    ///### 循环结构
    Loop(Arc<LoopAst>),
    For(Arc<ForAst>),
    While(Arc<WhileAst>),

    ///## 函数相关
    Defun(Arc<DefunAst>),
    FCall(Arc<FCallAst>),
}

impl ToString for Ast {
    fn to_string(&self) -> String {
        match self {
            Ast::Nil => "nil".to_string(),
            Ast::Bool(ref x) => x.to_string(),
            Ast::Char(ref x) => x.to_string(),
            Ast::Int(ref x) => x.to_string(),
            Ast::UInt(ref x) => x.to_string(),
            Ast::Float(ref x) => x.to_string(),
            /*
            Ast::String(ref x),
            Ast::Symbol(x),
            Ast::Tuple(x),
            Ast::Quote(x),
            Ast::Cond(x),
            Ast::Match(x),
            Ast::Loop(x),
            Ast::For(x),
            Ast::While(x),
            Ast::Defun(x),
            Ast::FCall(x),*/
            _ => "还没写完".to_string(),
        }
    }
}

trait Eval {
    fn eval(&self, ic :Arc<InterpreterContext>) -> Value;
}

trait CodeGen {
    fn code_gen_to_rust(&self) -> String;
    fn code_gen_to_dump(&self) -> Vec<u8>;
}
