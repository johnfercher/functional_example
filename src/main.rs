extern crate rand;
fn main() {
    let colecao_aleatoria: [u16; 8] = rand::random();
    let min = colecao_aleatoria
        .iter()
        .min()
        .unwrap();
    println!("{:?}", colecao_aleatoria);
    println!("{:?}", min);
}