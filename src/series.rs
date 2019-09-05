use ndarray::{Array1, Array};
use std::collections::HashSet;
use crate::element::Element;

#[derive(Debug, Clone)]
pub struct Series {
    pub label: String,
    pub data: Vec<String>
}

pub trait SeriesImpl {
    fn new() -> Series;
}

#[derive(Debug, Clone)]
pub enum TSeries {
    Text(Series),
}

impl SeriesImpl for Series {
    fn new() -> Series {
        Series {
            label: "".to_string(),
            data: Vec::new()
        }
    }
}
