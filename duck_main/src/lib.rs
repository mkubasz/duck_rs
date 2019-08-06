use rustlearn::prelude::*;
use rustlearn::linear_models::sgdclassifier::Hyperparameters;
use rustlearn::datasets::iris;
use std::fs::File;
use csv::{ReaderBuilder};
use ndarray::Array;
use std::error::Error;
use std::path::Path;
use std::env;
use std::any::Any;
use std::ops::IndexMut;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Element {
    value: String
}

#[derive(Debug, Clone)]
pub struct Column {
    pub data: Vec<Element>
}

impl Column {
    fn new() -> Column {
        Column {
            data: Vec::new()
        }
    }

    fn push(&mut self, element: Element) {

        self.data.push(element);
    }
}

#[derive(Debug)]
pub struct DataFrame {
    label: Vec<String>,
    data: Vec<Column>
}

impl DataFrame {
    pub fn new(size: usize) -> DataFrame {
        DataFrame {
            label: Vec::new(),
            data: vec![Column::new(); size]
        }
    }

    fn push(&mut self, element: Vec<Element>) {
        for (i, el) in element.iter().enumerate() {
            self.data[i].push(Element {
                value: el.value.clone()
            });
        }
    }

    pub fn by(&mut self, label: &str) -> &mut Column {
        let index = self.label.clone().iter().position(|el| el == label ).unwrap();
        &mut self.data[index]
    }
    pub fn read_csv(file_name: String) -> Result<(DataFrame), Box<dyn Error>> {
        let path = env::current_dir()?;
        println!("{:?}", path);
        let mut file = File::open(file_name)?;

        let mut rdr = csv::Reader::from_reader(file);
        let mut df = DataFrame::new(rdr.headers().unwrap().len());
        for header in rdr.headers() {
            for el in header.iter() {
                df.label.push(el.to_string());
            }
        }
        for result in rdr.records(){
            let mut row =Vec::new();
            let record = result?;
            for el in record.iter() {
                row.push(Element {value: el.to_string() });
            }
            df.push(row);
        }
        print!("{:?}", df);
        Ok(df)
    }
}