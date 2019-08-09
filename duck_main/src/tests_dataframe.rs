#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;
    use duck_main::dataframe::DataFrame;

    #[test]
    fn test_create_dataframe() {
        let df = DataFrame::read_csv(format!("src/Startups.csv")).unwrap();
        let mut profit = df["Profit"].clone();
        assert_eq!(df.size, 5);
    }

    #[test]
    fn test_create_dataframe_from_vec() {
        let df = DataFrame::new(3, None);
        assert_eq!(df.size, 3);
    }
}