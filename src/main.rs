extern crate rand;

fn main() {
    let numbers = vec![1, 2, 3, 4];

    let founded = numbers
        .clone()
        .into_iter()
        .find(|&x| x == 2)
        .unwrap();

    println!("{:?}", numbers);
    println!("{:?}", founded);
}