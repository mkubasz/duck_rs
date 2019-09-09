use num_traits::float::FloatCore;

use crate::cell::Cell;
use crate::series::Series;

fn minmax(minmax: &mut (f32, f32), cell: f32) -> () {
    if minmax.0.is_nan() {
        minmax.0 = cell;
        minmax.1 = cell;
    }else if minmax.0 >= cell {
        minmax.0 = cell;
    } else if minmax.1 < cell {
        minmax.1 = cell;
    }
}

fn series_minmax(series: Series<Cell>) -> (f32, f32) {
    let mut min_max: (f32, f32) = (f32::nan(), f32::nan());
    for el in series.data {
        match el {
            Cell::Integer(cell) => minmax(&mut min_max, cell as f32),
            Cell::Float(cell) => minmax(&mut min_max, cell),
            _ => {}
        }
    }
    min_max
}

#[cfg(test)]

mod tests {
    use crate::dataframe::DataFrame;
    use crate::dataframe::operations::Operations;
    use crate::utils::math::series_minmax;

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
        let (min, max) = series_minmax(series);
        assert_eq!(min, 0.4);
        assert_eq!(max, 3.0);
    }
}