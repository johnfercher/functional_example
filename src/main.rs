extern crate rand;

fn main() {
    let numbers = vec![1,2,3];

    let new_numbers = numbers
        .clone()
        .into_iter()
        .chain(numbers
            .clone()
            .into_iter())
        .collect::<Vec<u16>>();

    println!("{:?}", numbers);
    println!("{:?}", new_numbers);
}