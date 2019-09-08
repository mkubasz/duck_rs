use crate::types::{DInteger, DFloat, DataTypes};
use num_traits::{Float, NumCast, ToPrimitive};
use num::{PrimInt, Signed, Integer};
use std::iter::Product;

/// Basic elementary cell in data frame
#[derive(Debug, Clone, PartialEq)]
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

impl Into<String> for Element {
    fn into(self) -> String {
       match self {
           Element::Text(v) => v,
           _ => {panic!("Error")}
       }
    }
}

impl Into<u32> for Element {
    fn into(self) -> u32 {
        match self {
            Element::Integer(v) => v as u32,
            _ => {panic!("Error")}
        }
    }
}

impl Into<f32> for Element {
    fn into(self) -> f32 {
        match self {
            Element::Float(v) => v,
            _ => {panic!("Error")}
        }
    }
}

impl Into<bool> for Element {
    fn into(self) -> bool {
        match self {
            Element::Bool(v) => v,
            _ => {panic!("Error")}
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
