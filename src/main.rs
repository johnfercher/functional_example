extern crate rand;

fn main() {
    let tuples_array = [(1, 2), (3, 4)];

    let (left, right) : (Vec<u16>, Vec<u16>) = tuples_array.iter().cloned().unzip();

    println!("{:?}", tuples_array);
    println!("{:?}", left);
    println!("{:?}", right);
}