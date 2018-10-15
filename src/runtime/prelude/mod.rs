use std::cell::RefCell;
use std::collections::HashMap;
use std::process::exit;

use func::Native;
use value::Value;
use value::Handle;

use runtime::args_length_exception;
use runtime::context::ModuleContext;
use runtime::tools::exception;
use value::MaltResult;


pub fn system_module() -> ModuleContext {
    let mut vt: HashMap<String, Value> = HashMap::new();
    vt.insert(String::from("-module-name"), Value::Symbol(Handle::from(String::from("Prelude"))));

    vt.insert(String::from("-lang/version"), Value::Tuple(Handle::from(
        vec![Value::UInt(1), Value::UInt(0)])));

    vt.insert(String::from("true"), Value::Bool(true));
    vt.insert(String::from("false"), Value::Bool(false));

    // tool libs
    vt.insert(String::from("exit!"), Value::Native(Handle::from(Native {
        name: String::from("exit!"),
        fp: |_ic, args| {
            if args.len() == 0 {
                exit(0);
            } else if args.len() == 1 {
                if let Ok(x) = args[0].to_int() {
                    if let Value::Int(y) = x {
                        exit(y as i32);
                    }
                    // 这里不可能出现非Int的情况
                } else {
                    return Err(exception("TypeError", "'exit!' function call parameters type error"));
                }
            }
            return Err(args_length_exception());
        },
    })));

    vt.insert(String::from("+"), Value::Native(Handle::from(Native {
        name: String::from("+"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_add_once(s, v);
                    }
                }
                Ok(s)
            }
        },
    })));
    vt.insert(String::from("-"), Value::Native(Handle::from(Native {
        name: String::from("-"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_sub_once(s, v);
                    }
                }
                Ok(s)
            }
        },
    })));

    vt.insert(String::from("*"), Value::Native(Handle::from(Native {
        name: String::from("*"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_mul_once(s, v);
                    }
                }
                Ok(s)
            }
        },
    })));

    vt.insert(String::from("/"), Value::Native(Handle::from(Native {
        name: String::from("/"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_div_once(s, v)?;
                    }
                }
                Ok(s)
            }
        },
    })));

    vt.insert(String::from("not"), Value::Native(Handle::from(Native {
        name: String::from("not"),
        fp: |_ic, args| {
            if args.len() != 1 {
                return Err(exception("CallError", "Function 'not' call parameters size is not 1"));
            }
            if let Value::Bool(x) = args[0].clone() {
                return Ok(Value::Bool(!x));
            } else {
                return Err(exception("", ""));
            }
        },
    })));

    vt.insert(String::from("not"), Value::Native(Handle::from(Native {
        name: String::from("not"),
        fp: |_ic, args| {
            if args.len() != 1 {
                return Err(exception("CallError", "Function 'not' call parameters size is not 1"));
            }
            if let Value::Bool(x) = args[0].clone() {
                return Ok(Value::Bool(!x));
            } else {
                return Err(exception("", ""));
            }
        },
    })));

    ModuleContext {
        path: String::from("Prelude"),
        expr: Vec::new(),
        vtab: RefCell::from(vt),
    }
}

fn ex_add_once(s: Value, v: &Value) -> Value {
    match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            Value::Int((x + *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) + *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) + *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x + (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x + *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) + *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x + (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x + (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x + (*y as f64))
        }
        _ => {
            s
        }
    }
}

fn ex_sub_once(s: Value, v: &Value) -> Value {
    match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            Value::Int((x - *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) - *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) - *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x - (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x - *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) - *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x - (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x - (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x - (*y as f64))
        }
        _ => {
            s
        }
    }
}

fn ex_mul_once(s: Value, v: &Value) -> Value {
    match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            Value::Int((x * *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) * *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) * *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x * (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x * *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) * *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x * (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x * (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x * (*y as f64))
        }
        _ => {
            s
        }
    }
}

fn ex_div_once(s: Value, v: &Value) -> MaltResult {
    Ok(match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            if *y == 0 {
                return Err(exception("ZeroDivisionError", "division by zero"));
            }
            Value::Int((x / *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) / *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) / *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x / (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x / *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) / *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x / (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x / (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x / (*y as f64))
        }
        _ => {
            s
        }
    })
}