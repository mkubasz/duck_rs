#[cfg(test)]
mod tests {
    use super::*;
    use duck_main::DataFrame;
    use std::any::Any;

    #[test]
    fn test_create_dataframe() {
        let df = DataFrame::read_csv(format!("src/Startups.csv")).unwrap();
        assert_eq!(df.size, 5);
    }

    #[test]
    fn test_create_dataframe_from_vec() {
        let df = DataFrame::new(3, None);
        assert_eq!(df.size, 3);
    }
}