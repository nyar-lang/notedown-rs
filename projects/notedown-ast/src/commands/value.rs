use std::collections::HashMap;
use crate::AST;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    String(String),
    Integer(String),
    Decimal(String),
    Boolean(bool),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Command(AST),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, ""),
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(s) => write!(f, "{}", s),
            Value::Decimal(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(_) => { unimplemented!() }
            Value::Dict(_) => { unimplemented!() }
            Value::Command(_) => { unimplemented!() }
        }
    }
}
