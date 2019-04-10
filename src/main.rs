extern crate rand;

fn main() {
    let colecao_aleatoria: [u16; 8] = rand::random();

    let max = colecao_aleatoria
        .iter()
        .max()
        .unwrap();

    println!("{:?}", colecao_aleatoria);
    println!("{:?}", max);
}