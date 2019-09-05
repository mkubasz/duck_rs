#[cfg(test)]

mod tests {
    use crate::dataframe::{DataFrame, DataFrameImpl};

    #[test]
    fn test_create_dataframe() {
        //let df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        //let profit = df["Profit"].clone();
        assert_eq!(1, 5);
    }

    #[test]
    fn test_create_dataframe_from_vec() {
        let df = DataFrame::new(
            3,
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ]
        );
        assert_eq!(df.size, 3);
    }
}