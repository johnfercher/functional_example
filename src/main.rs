extern crate rand;

fn main() {
    let numbers = vec![1,2,3,4,5,6];
    let mut sum = 0;

    numbers
        .to_vec()
        .into_iter()
        .for_each(|x| {
            sum = sum + x;
        });

    println!("{:?}", numbers);
    println!("{:?}", sum);
}