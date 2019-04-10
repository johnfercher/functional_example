extern crate rand;

fn main() {
    let all_odds = vec![1, 3];
    let not_all_odds = vec![1, 2, 3];

    let has_any_odd1 = all_odds
        .clone()
        .into_iter()
        .any(|x| x % 2 == 0);

    let has_any_odd2 = not_all_odds
        .clone()
        .into_iter()
        .any(|x| x % 2 == 0);

    println!("{:?}", all_odds);
    println!("{:?}", not_all_odds);
    println!("{:?}", has_any_odd1);
    println!("{:?}", has_any_odd2);
}