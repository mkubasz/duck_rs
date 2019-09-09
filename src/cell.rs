use crate::types::{DataTypes, DFloat, DInteger};

/// Basic elementary cell in data frame
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Text(String),
    Integer(i32),
    Float(f32),
    Bool(bool)
}

impl Cell {
    pub fn data_type(&self) -> DataTypes {
        match self {
            Cell::Text(_) => DataTypes::Text,
            Cell::Integer(_) => DataTypes::Integer,
            Cell::Float(_) => DataTypes::Float,
            Cell::Bool(_) => DataTypes::Bool,
            _ => DataTypes::Text
        }
    }

}

impl Into<String> for Cell {
    fn into(self) -> String {
       match self {
           Cell::Text(v) => v,
           _ => {panic!("Error")}
       }
    }
}

impl Into<u32> for Cell {
    fn into(self) -> u32 {
        match self {
            Cell::Integer(v) => v as u32,
            _ => {panic!("Error")}
        }
    }
}

impl Into<f32> for Cell {
    fn into(self) -> f32 {
        match self {
            Cell::Float(v) => v,
            _ => {panic!("Error")}
        }
    }
}

impl Into<bool> for Cell {
    fn into(self) -> bool {
        match self {
            Cell::Bool(v) => v,
            _ => {panic!("Error")}
        }
    }
}

impl From<String> for Cell {
    fn from(v: String) -> Self {
        Cell::Text(v)
    }
}

impl From<DInteger> for Cell {
    fn from(v: DInteger) -> Self {
        Cell::Integer(v)
    }
}

impl From<DFloat> for Cell {
    fn from(v: DFloat) -> Self {
        Cell::Float(v)
    }
}

impl From<bool> for Cell {
    fn from(v: bool) -> Self {
        Cell::Bool(v)
    }
}

impl From<&str> for Cell {
    fn from(v: &str) -> Self {
        Cell::Text(v.to_owned())
    }
}
