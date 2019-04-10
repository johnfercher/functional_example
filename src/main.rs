extern crate rand;

fn main() {
    let numbers = vec![1,2,3,4,5,6];

    let sum: u32 = numbers
        .iter()
        .product();

    println!("{:?}", numbers);
    println!("{:?}", sum);
}