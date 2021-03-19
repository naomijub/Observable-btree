use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Char(char),
    Integer(isize),
    UInteger(usize),
    String(String),
    Float(f64),
    Boolean(bool),
    Vector(Vec<Types>),
    HashMap(HashMap<String, Types>),
    BTreeMap(BTreeMap<String, Types>),
    KeyValue(String, Box<Types>),
    Nil,
}
