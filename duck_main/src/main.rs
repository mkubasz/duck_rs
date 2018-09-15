mod lib;

fn main() {
    let mut dck = lib::duck::Duck::new();
    let obj = dck.array(&[1, 2, 3]);
    print!("Data: {:?}\n", obj.data);
    print!("Size: {}\n", obj.size);
    print!("Shape: {}\n", obj.shape);
    print!("Type: {}\n", obj.d_type);
}
