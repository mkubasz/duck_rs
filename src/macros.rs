#[macro_export]
macro_rules! row {
    ( $( $x:expr ),* ) => {
        {
            use crate::cell::Cell;
            let mut temp_vec = Vec::<Cell>::new();
            $(
                temp_vec.push(Cell::from($x));
            )*
            temp_vec
        }
    };
}