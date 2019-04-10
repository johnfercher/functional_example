extern crate rand;

fn main() {
    let all_evens = vec![2, 4];
    let not_all_evens = vec![2, 4, 3];

    let is_all_even1 = all_evens
        .clone()
        .into_iter()
        .all(|x| x % 2 == 0);

    let is_all_even2 = not_all_evens
        .clone()
        .into_iter()
        .all(|x| x % 2 == 0);

    println!("{:?}", all_evens);
    println!("{:?}", not_all_evens);
    println!("{:?}", is_all_even1);
    println!("{:?}", is_all_even2);
}