#[macro_export]
macro_rules! row {
    ( $( $x:expr ),* ) => {
        {
            use crate::element::Element;
            let mut temp_vec = Vec::<Element>::new();
            $(
                temp_vec.push(Element::from($x));
            )*
            temp_vec
        }
    };
}