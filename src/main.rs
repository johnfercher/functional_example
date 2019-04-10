extern crate rand;

fn main() {
    let colecao_aleatoria: [u16; 8] = rand::random();

    let colecao_aleatoria_invertida  = colecao_aleatoria
        .to_vec()
        .into_iter()
        .rev()
        .collect::<Vec<u16>>();

    println!("{:?}", colecao_aleatoria);
    println!("{:?}", colecao_aleatoria_invertida);
}