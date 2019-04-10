extern crate rand;

fn main() {
    let numbers = vec![1,2,3];

    let enumerate_numbers = numbers
        .clone()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, u16)>>();

    println!("{:?}", numbers);
    println!("{:?}", enumerate_numbers);
}