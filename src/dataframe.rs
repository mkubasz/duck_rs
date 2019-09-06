use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ops::{Index, IndexMut};

use crate::series::{Series, SeriesImpl, TSeries};
use crate::element::{Element};
use criterion::Throughput::Elements;
use crate::types::DataTypes;
use std::borrow::Borrow;


#[derive(Debug, Clone)]
pub struct DataFrame {
    pub size: usize,
    labels: Vec<String>,
    data: Vec<TSeries>,
}

pub trait DataFrameImpl {
    fn new(vec: Vec<Vec<Element>>, labels: Vec<&str>) -> DataFrame;
    fn push(&mut self, element: Vec<Element>);
    fn series(&mut self, index:  usize) -> &mut Series;
    /// Get selected column by using label name
    fn by(&mut self, label: &str) -> &mut Series;
    /// Get selected column by using label name
    fn many(&mut self, labels: Vec<&str>) -> Vec<Series>;
    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame;
    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>>;
}

impl DataFrameImpl for DataFrame {
    fn new(vec: Vec<Vec<Element>>, labels: Vec<&str>) -> DataFrame {
        let mut column_types = vec![];
        // Figure out the column types from the data
        for i in 0..vec[0].len() {
            column_types.push(vec[0][i].data_type());
        }

        // create columns based on column types
        let mut cols = Vec::<TSeries>::new();
        for (i, t) in column_types.iter().enumerate() {
            match t {
                DataTypes::Text => cols.push(TSeries::Text(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                })),
                _ => cols.push(TSeries::Text(Series {
                    label: labels[i].to_string(),
                    data: Vec::new(),
                })),
            }
        }

        let mut size = 0;
        for row in vec.iter() {
            for (col_index, cell) in row.iter().enumerate() {
                match &mut cols[col_index] {
                    TSeries::Text(col) => match &cell {
                        Element::Text(val) => col.data.push(val.clone()),
                        Element::Integer(val) => col.data.push(format!("{}", val.clone())),
                        Element::Float(val) => col.data.push(format!("{}", val.clone())),
                        Element::Bool(val) => col.data.push(format!("{}", val.clone())),
                    },
                    _=> {},
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
                match &mut self.data[i] {
                TSeries::Text(col) => match &el {
                    Element::Text(val) => col.data.push(val.clone()),
                    Element::Integer(val) => col.data.push(format!("{}", val.clone())),
                    Element::Float(val) => col.data.push(format!("{}", val.clone())),
                    Element::Bool(val) => col.data.push(format!("{}", val.clone())),
                },
                _=> {},
            }
            self.size += 1;
        }
    }

    fn series(&mut self, index: usize) -> &mut Series {
        match &mut self.data[index] {
            TSeries::Text(col) => col
        }
    }

    fn by(&mut self, label: &str) -> &mut Series {
        let index = self.labels.clone().iter()
            .position(|el| el == label)
            .unwrap();
        self.series((index).to_owned())
    }

    fn many(&mut self, labels: Vec<&str>) -> Vec<Series> {
        let mut vec: Vec<Series> = Vec::new();
        for index in 0..self.data.len() {
            let series = self.series(index).clone();
            if labels.contains(&series.label.as_str()) {
                vec.push(series.to_owned());
            }
        }
        vec
    }

    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame {
        for el in &mut self.by(col).data {
            for (key, v) in obj.iter() {
                if *key == el.as_str() {
                    break;
                }
            }
        }
        self.to_owned()
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
