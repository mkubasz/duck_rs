mod lib;
use self::lib::duck::Vector;
use self::lib::duck::Column;
use self::lib::duck::Matrix;

fn main() {
    let obj = Vector::new(&[1, 2, 3]);
    print!("Data: {:?}\n", obj.data());
    print!("Size: {}\n", obj.size());
    print!("Shape: {}\n", obj.shape());
    print!("Type: {}\n", obj.d_type());

    let obj2 = Matrix::new(
        vec![
            Column::new(&[1,2,3]),
            Column::new(&[1,2,3]),
            Column::new(&[1,2,3])
        ]
    );
    print!("Data: {:?}\n", obj2.data());
    print!("Size: {}\n", obj2.size());
    print!("Shape: {}\n", obj2.shape());
    print!("Type: {}\n", obj2.d_type());
}
