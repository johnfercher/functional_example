extern crate rand;

fn main() {
    let misc = ["1", "batata", "3", "thor", "5"];

    let numbers = misc
        .iter()
        .filter_map(|x| x.parse().ok() )
        .collect::<Vec<u16>>();

    println!("{:?}", misc);
    println!("{:?}", numbers);
}