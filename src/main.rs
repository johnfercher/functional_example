extern crate rand;

fn main() {
    let numbers = vec![1, 2, 3];

    let new_numbers = numbers
        .clone()
        .into_iter()
        .take(1)
        .collect::<Vec<u16>>();

    println!("{:?}", numbers);
    println!("{:?}", new_numbers);
}