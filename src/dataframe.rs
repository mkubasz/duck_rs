use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ops::{Index, IndexMut};

use crate::series::{Series, SeriesImpl, TSeries};
use crate::element::{Element};
use criterion::Throughput::Elements;
use crate::types::DataTypes;


#[derive(Debug, Clone)]
pub struct DataFrame {
    pub size: usize,
    labels: Vec<String>,
    data: Vec<TSeries>,
}

pub trait DataFrameImpl {
    fn new(size: usize, vec: Vec<Vec<Element>>) -> DataFrame;
}

impl DataFrameImpl for DataFrame {
    fn new(size: usize, vec: Vec<Vec<Element>>) -> DataFrame {
        let mut column_types = vec![];

        // Figure out the column types from the data
        if vec.len() > 0 {
            for i in 0..3 {
                if i >= vec[0].len() {
                    column_types.push(DataTypes::Integer);
                } else {
                    column_types.push(vec[0][i].data_type());
                }
            }
        } else {
            for _ in 0..3 {
                column_types.push(DataTypes::Integer);
            }
        }

        // create columns based on column types
        let mut cols = Vec::<TSeries>::new();
        for (i, v) in column_types.iter().enumerate() {
            match v {
                DataTypes::Text => cols.push(TSeries::Text(Series {
                    label: format!(""),
                    data: vec![],
                })),
                _ => {}
            }
        }

        for row in vec.iter() {

            for (col_index, cell) in row.iter().enumerate() {
                match &mut cols[col_index] {
                    TSeries::Text(col) => match &cell {
                        Element::Text(val) => col.data.push(val.clone()),
                        _ => {
                        }
                    },
                    _ => {}
                }
            }
        }

        DataFrame {
            size,
            labels: Vec::new(),
            data: cols,
        }
    }
}
