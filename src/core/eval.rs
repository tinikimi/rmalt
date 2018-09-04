use ast::Ast;
use std::sync::Arc;
use core::interpreter::ThreadContext;
use value::Value;
use ast::AstValue;

impl Ast {
    pub fn eval(&self) -> Value {
        //fn eval(&self, ic: Arc<ThreadContext>) -> Value {
        match self.val {
            AstValue::Nil => Value::Nil,
            AstValue::Bool(ref x) => Value::Bool(x.clone()),
            AstValue::Char(ref x) => Value::Char(x.clone()),
            AstValue::Int(ref x) => Value::Int(x.clone()),
            AstValue::UInt(ref x) => Value::UInt(x.clone()),
            AstValue::Float(ref x) => Value::Float(x.clone()),
            AstValue::String(ref x) => Value::String(x.clone()), // FIXME:???
            AstValue::Symbol(ref x) => Value::Symbol(x.clone()), // TODO:!!!
            AstValue::Quote(ref x) => Value::Ast(Arc::from(x.expr.clone())),
            AstValue::Tuple(ref x) => Value::String(Arc::from(x.to_string())), // TODO:!!!
            /*
            AstValue::Tuple(x),
            AstValue::Cond(x),
            AstValue::Match(x),
            AstValue::Defun(x),
            AstValue::FCall(x),*/
            _ => Value::Nil, //"还没写完".to_string(),
        }
    }
}