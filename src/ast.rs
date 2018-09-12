use std::sync::Arc;
use value::Value;
use value::_Str;
use core::interpreter::InterpreterContext;
use parser::token::TokenPos;
use core::interpreter::ThreadContext;


/*
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
    pub cval: Ast,
    // cond value
    pub pair: Vec<(Ast, Ast)>,
}

pub struct LoopAst {
    pub expr: Vec<Ast>,
}

pub struct ForAst {
    pub name: String,
    pub tuple: Ast,
    // cond value
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
*/

pub struct List {
    pub list: Vec<Ast>,
}

pub enum AstValue {
    ///## 字面量
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    UInt(u64),
    Float(f64),

    String(_Str),
    Symbol(_Str),

    List(Arc<List>),

    /*
    ///### 元组
    Tuple(Arc<TupleAst>),
    ///### 引用
    Quote(Arc<QuoteAst>),
    ///## 控制结构
    ///### 分支结构
    Cond(Arc<CondAst>),
    Match(Arc<MatchAst>),
    ///## 函数相关
    Defun(Arc<DefunAst>),
    FCall(Arc<FCallAst>),
    */
}

pub struct Ast {
    pub val: AstValue,
    pub pos: TokenPos,
}

impl Clone for AstValue {
    fn clone(&self) -> AstValue {
        match self {
            AstValue::Nil => AstValue::Nil,
            AstValue::Bool(ref x) => AstValue::Bool(x.clone()),
            AstValue::Char(ref x) => AstValue::Char(x.clone()),
            AstValue::Int(ref x) => AstValue::Int(x.clone()),
            AstValue::UInt(ref x) => AstValue::UInt(x.clone()),
            AstValue::Float(ref x) => AstValue::Float(x.clone()),
            AstValue::String(ref x) => AstValue::String(x.clone()),
            AstValue::Symbol(ref x) => AstValue::Symbol(x.clone()),
            AstValue::List(ref x) => AstValue::List(x.clone()),
            /*
            AstValue::Tuple(ref x) => AstValue::Tuple(x.clone()),
            AstValue::Quote(ref x) => AstValue::Quote(x.clone()),
            AstValue::Cond(ref x) => AstValue::Cond(x.clone()),
            AstValue::Match(ref x) => AstValue::Match(x.clone()),
            AstValue::Defun(ref x) => AstValue::Defun(x.clone()),
            AstValue::FCall(ref x) => AstValue::FCall(x.clone()),
            */
        }
    }
}

impl Clone for Ast {
    fn clone(&self) -> Ast {
        Ast {
            val: self.val.clone(),
            pos: self.pos.clone(),
        }
    }
}


impl ToString for List {
    fn to_string(&self) -> String {
        let mut rs = String::from("[");
        for i in &self.list {
            rs += &i.to_string();
            rs += " ";
        }
        rs.push(']');
        rs
    }
}


impl ToString for Ast {
    fn to_string(&self) -> String {
        match self.val {
            AstValue::Nil => "nil".to_string(),
            AstValue::Bool(ref x) => match x {
                true => "bool: true".to_string(),
                false => "bool: false".to_string(),
            },
            AstValue::Char(ref x) => "char: ".to_string() + &x.to_string(),
            AstValue::Int(ref x) => "int: ".to_string() + &x.to_string(),
            AstValue::UInt(ref x) => "uint: ".to_string() + &x.to_string(),
            AstValue::Float(ref x) => "float: ".to_string() + &x.to_string(),
            AstValue::String(ref x) => "string: ".to_string() + &x.to_string(),
            AstValue::Symbol(ref x) => "symbol: ".to_string() + &x.to_string(),
            /*
            AstValue::Quote(ref x) => "quote: ".to_string() + &x.expr.to_string(),
            AstValue::Tuple(ref x) => x.to_string(),
            AstValue::Cond(x),
            AstValue::Match(x),
            AstValue::Defun(x),
            AstValue::FCall(x),*/
            _ => "还没写完".to_string(),
        }
    }
}

trait CodeGen {
    fn code_gen_to_rust(&self) -> String;
    //fn code_gen_to_dump(&self) -> Vec<u8>;
}
