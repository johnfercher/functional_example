extern crate rand;

fn main() {
    let numbers = vec![1,2,3];
    let matrix = vec![numbers.clone(), numbers.clone(), numbers.clone()];

    let flat_matrix = matrix
        .clone()
        .into_iter()
        .flat_map(|x| x)
        .collect::<Vec<u16>>();

    println!("{:?}", matrix);
    println!("{:?}", flat_matrix);
}