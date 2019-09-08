use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ops::{Index, IndexMut};

use crate::series::{Series, SeriesImpl, TSeries};
use crate::element::{Element};
use criterion::Throughput::Elements;
use crate::types::DataTypes;
use std::borrow::Borrow;
use num::{Num, NumCast, PrimInt, Unsigned};
use num_traits::{Float, Zero, Signed};
use std::iter::Product;

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub size: usize,
    labels: Vec<String>,
    data: Vec<Series<Element>>,
}

pub trait DataFrameImpl {
    fn new(vec: Vec<Vec<Element>>, labels: Vec<&str>) -> DataFrame;
    fn push(&mut self, element: Vec<Element>);
    fn series(&mut self, index:  usize) -> &mut Series<Element>;
    /// Get selected column by using label name
    fn by(&mut self, label: &str) -> Option<&mut Series<Element>>;
    /// Get selected column by using label name
    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Element>>;
    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame;
    /// Concatenate two data frames
    fn concat(&mut self, df: DataFrame) -> DataFrame;
    /// Drop column by label from Data Frame
    fn drop(&mut self, label: Vec<&str>) -> Option<DataFrame>;
    /// Drop column by position from Data Frame
    fn drop_idx(&mut self, position: usize) -> Option<DataFrame>;
    fn contains(self, label: &str) -> bool;
    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>>;
}

pub trait DataFrameScienceImpl {
    /// One hot encoding - Convert string values to binary value
    fn get_dummies(&mut self, label: &str) -> DataFrame;
    fn from_vec<T>(vec: Vec<Vec<T>>, labels: Vec<&str>) -> DataFrame where T: PrimInt + Signed + Product;
}

impl DataFrameImpl for DataFrame {
    fn new(vec: Vec<Vec<Element>>, labels: Vec<&str>) -> DataFrame {
        let mut column_types = vec![];
        // Figure out the column types from the data
        for i in 0..vec[0].len() {
            column_types.push(vec[0][i].data_type());
        }

        // create columns based on column types
        let mut cols = Vec::<Series<Element>>::new();
        for (i, t) in column_types.iter().enumerate() {
            match t {
                DataTypes::Text => cols.push(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                }),
                DataTypes::Float => cols.push(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                }),
                _ => cols.push(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                }),
            }
        }

        let mut size = 0;
        for row in vec.iter() {
            for (col_index, cell) in row.iter().enumerate() {
                match cell {
                    Element::Float(c) => {
                        cols[col_index].data.push(cell.clone());
                    }
                    _ => {
                        cols[col_index].data.push(cell.clone());
                    }
                }
            }
            size += 1;
        }
        let mut tmp: Vec<String> = Vec::new();
        for label in labels {
            tmp.push(label.to_string())
        }
        DataFrame {
            size,
            labels: tmp,
            data: cols,
        }
    }

    fn push(&mut self, vec: Vec<Element>) {
        for (i, el) in vec.iter().enumerate() {
            self.data[i].data.push(el.clone());
        }
        self.size += 1;
    }

    fn series(&mut self, index: usize) -> &mut Series<Element> {
        &mut self.data[index]
    }

    fn by(&mut self, label: &str) -> Option<&mut Series<Element>> {
        let index = self.labels.clone().iter()
            .position(|el| el == label)
            .unwrap();
        Some(self.series((index).to_owned()))
    }

    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Element>> {
        let mut vec: Vec<Series<Element>> = Vec::new();
        for index in 0..self.data.len() {
            let series = self.series(index).clone();
            if labels.contains(&series.label.as_str()) {
                vec.push(series.to_owned());
            }
        }
        vec
    }

    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame {
        for el in &mut self.by(col).unwrap().data {
            for (key, v) in obj.iter() {
                match el {
                    Element::Text(cell) => {
                        if *key == cell {
                            *el = Element::from(format!("{}", v));
                            break;
                        }
                    }
                    _ => {}
                }

            }
        }
        self.to_owned()
    }

    fn concat(&mut self, df: DataFrame) -> DataFrame {
        DataFrame {
            size: self.labels.len()+df.labels.len(),
            labels: [&self.labels[..], &df.labels[..]].concat(),
            data: [&self.data[..], &df.data[..]].concat(),
        }
    }

    fn drop(&mut self, labels: Vec<&str>) -> Option<DataFrame> {
        for label in labels {
            let position = self.labels.clone().iter().position(
                |el| el == label
            ).unwrap();
            self.labels.remove(position);
            self.data.remove(position);
        }
        let a = self.to_owned();
        Some(a)
    }

    fn drop_idx(&mut self, position: usize) -> Option<DataFrame> {
        if self.labels.len() < position {
            return None;
        }
        self.labels.remove(position);
        self.data.remove(position);
        Some(self.to_owned())
    }

    fn contains(self, label: &str) -> bool {
        self.labels.contains(&label.to_string())
    }

    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>> {
        let file = File::open(file_name)?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut vec: Vec<Vec<Element>> = Vec::new();
        for result in rdr.records() {
            let mut row: Vec<Element> = Vec::new();
            let record = result?;
            for el in record.iter() {
                row.push(Element::from(el.clone()));
            }
            vec.push(row);
        }

        let mut labels = Vec::new();
        for header in rdr.headers() {
            for el in header.iter() {
                labels.push(el.clone())
            }
        }
        Ok(DataFrame::new(vec, labels.clone()))
    }
}

impl Index<&str> for DataFrame {
    type Output = Series<Element>;

    fn index(&self, label: &str) -> &Self::Output {
        for col in &self.data {
            if col.label == label {
                return &col;
            }
        }
        panic!("unknown column name")
    }
}

impl Index<usize> for DataFrame {
    type Output = Series<Element>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for DataFrame {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.series(i)
    }
}

impl DataFrameScienceImpl for DataFrame {
    fn get_dummies(&mut self, label: &str) -> DataFrame {
        let column = self.by(label.clone()).unwrap();
        let unique_column = column.clone().unique();
        let size = unique_column.clone().data.len();
        let columns: Vec<Vec<Element>> = column.data.iter().map(|el| {
            let mut tmp = vec![Element::Integer(0); size];

            let index = unique_column.data.iter().position(
                |it| *it == *el
            ).unwrap();
            tmp[index] = Element::Integer(1);
            tmp
        }).collect();
        let mut new_labels = Vec::new();
        for i in 0..size {
            let mut temp = format!("{}_{}", label, i);
            new_labels.push(temp.clone());
        }
        let v2: Vec<&str> = new_labels.iter().map(|s| &**s).collect();
        let mut df = DataFrame::new(columns, v2);
        df
    }

    fn from_vec<T: NumCast + Copy>(vec: Vec<Vec<T>>, labels: Vec<&str>) -> DataFrame {
        let mut new_vec: Vec<Vec<Element>> = Vec::new();
        for columns in vec.iter() {
            let mut elements: Vec<Element> = Vec::new();
            for  value in columns.iter() {
                elements.push(Element::from(num::cast::<_, i32>(*value).unwrap()));
            }
            new_vec.push(elements);
        }
        DataFrame::new(new_vec.clone(), labels)
    }
}
