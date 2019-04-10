extern crate rand;

fn main() {
    let numbers = vec![1,2,3,4,5,6];
    let numbers2 = vec![6,5,4,3,2,1];

    let tuples = numbers
        .clone()
        .into_iter()
        .zip(numbers2
            .clone()
            .into_iter())
        .collect::<Vec<(u16, u16)>>();

    println!("{:?}", numbers);
    println!("{:?}", numbers2);
    println!("{:?}", tuples);
}