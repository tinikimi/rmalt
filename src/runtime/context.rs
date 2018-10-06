use std::sync::Arc;
use std::sync::RwLock;
use std::cell::RefCell;
use std::collections::HashMap;

use value::Value;
use value::_Str;
use value::_Function;

pub struct ModuleContext {
    pub path: String,
    pub expr: Vec<Value>,
    // 其实可以不用这个字段的，但我要codegen
    pub vtab: HashMap<String, Value>, // env (var table)
}

pub struct FunctionContext {
    pub fun: _Function,
    pub vtab: RefCell<HashMap<String, Value>>, // env (var table)
}

pub type CommonModuleContext = RefCell<HashMap<String, Arc<ModuleContext>>>;

pub struct ThreadContext {
    pub commonmod: Arc<RwLock<CommonModuleContext>>,
    pub using_mod: Arc<ModuleContext>,
    pub framestack: RefCell<Vec<RefCell<Option<Arc<FunctionContext>>>>>,
    pub frame_size: RefCell<usize>,
}

impl ThreadContext {
    pub fn load_symbol(&self, sym: _Str) -> Option<Value> {
        // 首先看看函数里边有没有
        if self.frame_size.borrow().clone() != 0 {
            let a = &self.framestack.borrow();
            let b = a[self.frame_size.borrow().clone() - 1].borrow().clone().unwrap();
            let c = b.vtab.borrow();
            let d = c.get(sym.as_ref());
            if let Some(x) = d {
                return Some((*x).clone());
            }
            // 写完以上后陷入沉思。。。
        }
        // 然后再看看本模块有没有
        // using_mod
        None
    }
}