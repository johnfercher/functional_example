extern crate rand;

fn main() {
    let colecao_aleatoria: [u16; 8] = rand::random();

    let (pares, impares): (Vec<u16>, Vec<u16>) = colecao_aleatoria
        .into_iter()
        .partition(|&n| n % 2 == 0);

    println!("{:?}", colecao_aleatoria);
    println!("{:?}", pares);
    println!("{:?}", impares);
}