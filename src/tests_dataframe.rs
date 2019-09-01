#[cfg(test)]

mod tests {
    use crate::dataframe::{DataFrame, DataFrameImpl};
    
    #[test]
    fn test_create_dataframe() {
        let df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let profit = df["Profit"].clone();
        assert_eq!(df.size, 5);
    }

    #[test]
    fn test_create_dataframe_from_vec() {
        let df = DataFrame::new(3, None);
        assert_eq!(df.size, 3);
    }
}