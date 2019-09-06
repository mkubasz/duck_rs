#[cfg(test)]

mod tests {
    use crate::dataframe::{DataFrame, DataFrameImpl};
    use std::ops::Deref;

    #[test]
    fn test_create_dataframe_from_vec() {
        let df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]

        );
        assert_eq!(df.size, 3);
    }

    #[test]
    fn test_push_dataframe_from_vec() {
        let mut df = DataFrame::new(
            vec![
                row![0.4, 0.7, "book", true, 1],
                row![3.0, 4.7, "poster", true, 1],
            ],
            vec!["A", "B", "C", "D", "E"]
        );
        df.push(row![0.4, 0.7, "book", true, 1]);
        assert_eq!(df.size, 4);
    }

    #[test]
    fn test_create_from_csv_dataframe() {
        let df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        assert_eq!(df.size, 50);
    }

    #[test]
    fn test_read_by() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let series = df.by("Profit");
        assert_eq!(series.label, "Profit".to_string());
    }

    #[test]
    fn test_read_many() {
        let mut df = DataFrame::read_csv(format!("src/data/Startups.csv")).unwrap();
        let series = df.many(vec!["Profit", "State"]);
        assert_eq!(series.len(), 2);
    }
}