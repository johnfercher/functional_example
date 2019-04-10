extern crate rand;

fn main() {
    let misc = vec!["batman", "batata", "3", "thor"];

    let founded : u16 = misc
        .clone()
        .into_iter()
        .find_map(|x| x.parse().ok())
        .unwrap();

    println!("{:?}", misc);
    println!("{:?}", founded);
}