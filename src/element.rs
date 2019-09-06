use crate::types::{DInteger, DFloat, DataTypes};

/// Basic elementary cell in data frame
#[derive(Debug)]
pub enum Element {
    Text(String),
    Integer(i32),
    Float(f32),
    Bool(bool)
}

impl Element {
    pub fn data_type(&self) -> DataTypes {
        match self {
            Element::Text(_) => DataTypes::Text,
            Element::Integer(_) => DataTypes::Integer,
            Element::Float(_) => DataTypes::Float,
            Element::Bool(_) => DataTypes::Bool,
            _ => DataTypes::Text
        }
    }
}

impl From<String> for Element {
    fn from(v: String) -> Self {
        Element::Text(v)
    }
}

impl From<DInteger> for Element {
    fn from(v: DInteger) -> Self {
        Element::Integer(v)
    }
}


impl From<DFloat> for Element {
    fn from(v: DFloat) -> Self {
        Element::Float(v)
    }
}

impl From<bool> for Element {
    fn from(v: bool) -> Self {
        Element::Bool(v)
    }
}


impl From<&str> for Element {
    fn from(v: &str) -> Self {
        Element::Text(v.to_owned())
    }
}