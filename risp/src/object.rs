#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    lambda(Vec<String>,Vec<Object>),
    List(Vec<Object>)
}


