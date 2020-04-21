use num_traits::float::Float;
use num_traits::int::PrimInt;

use crate::cell::Cell;
use crate::series::Series;

use std::f64;
use num_traits::FromPrimitive;

fn sigmoid(x: f64) -> f64 {
    x.signum()
}

fn minmax_item<T: Into<f64> + Copy>(min_max: (f64, f64), item: T) -> (f64, f64) {
    (
        min_max.0.min(item.into()),
        min_max.1.max(item.into())
    )
}

fn minmax<T: Into<f64> + Copy>(series: Vec<T>) -> Option<(f64, f64)> {
    if series.is_empty() {
        ()
    }
    let mut min_max = (series[0].into(), series[0].into());
    series
        .iter()
        .skip(1)
        .for_each(
            | &element| {
                min_max = minmax_item(min_max, element);
            });
    Some(min_max)
}

fn series_minmax(series: Series<Cell>) -> (f64, f64) {
    let mut min_max: (f64, f64) = (f64::nan(), f64::nan());
    for el in series.data {
        match el {
            Cell::Integer(cell) => {
                min_max = minmax_item(min_max, cell);
            },
            Cell::Float(cell) => {
                min_max = minmax_item(min_max, cell);
            },
            Cell::Float64(cell) => {
                min_max = minmax_item(min_max, cell);
            },
            _ => {}
        }
    }
    min_max
}

#[cfg(test)]
mod tests {
    use crate::math::math;
    use crate::dataframe::DataFrame;
    use crate::dataframe::operations::Operations;
    #[test]
    fn test_minmax_float() {
        let series = vec![0.2, 5.6, 7.0, 3.0];
        let (min, max) = math::minmax(series).unwrap();
        assert_eq!(min, 0.2);
        assert_eq!(max, 7.0);
    }

    #[test]
    fn test_minmax_integer() {
        let series = vec![2, 5, 7, 0];
        let (min, max) = math::minmax(series).unwrap();
        assert_eq!(min, 0.0);
        assert_eq!(max, 7.0);
    }

    #[test]
    fn test_minmax_series() {
        let series = vec![2, 5, 7, 0];
        let (min, max) = math::minmax(series).unwrap();
        assert_eq!(min, 0.0);
        assert_eq!(max, 7.0);
    }
    #[test]
    fn test_series_minmax() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        let series = df.by("A").unwrap().to_owned();
        let (min, max) = math::series_minmax(series);
        assert_eq!(min, 0.4);
        assert_eq!(max, 3.0);
    }
}
