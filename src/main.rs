extern crate rand;

fn main() {
    let numbers = vec![1,2,3,4,5,6];

    let sum: u32 = numbers
        .iter()
        .fold(0, |mut sum, &val| {sum += val; sum});

    println!("{:?}", numbers);
    println!("{:?}", sum);
}