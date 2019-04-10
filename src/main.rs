extern crate rand;

fn main() {
    let numbers = vec![1,2,3,4,5,6];
    let numbers2 = vec![1,2,3,4,5,6];

    let contains_any_umatched_numbers = numbers
        .iter()
        .zip(numbers2.iter())
        .map(|(num1, num2)| num1 == num2)
        .collect::<Vec<bool>>()
        .contains(&false);

    println!("{:?}", numbers);
    println!("{:?}", numbers2);
    println!("{:?}", contains_any_umatched_numbers);
}