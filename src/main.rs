extern crate rand;

fn main() {
    let numbers = vec![1, 2, 3, 2, 4];

    let position  = numbers
        .clone()
        .into_iter()
        .position(|x| x == 2)
        .unwrap();

    let rposition  = numbers
        .clone()
        .into_iter()
        .rposition(|x| x == 2)
        .unwrap();

    println!("{:?}", numbers);
    println!("{:?}", position);
    println!("{:?}", rposition);
}