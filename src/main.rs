extern crate rand;

fn main() {
    let colecao_aleatoria: [u16; 32] = rand::random();

    let numbers = colecao_aleatoria
        .to_vec()
        .into_iter()
        .map(|x| {
            if x % 2 == 0 {
                String::from(format!("Número par: {:?}", x))
            } else {
                String::from(format!("Número impar: {:?}", x))
            }
        })
        .collect::<Vec<String>>();

    println!("{:?}", colecao_aleatoria);
    println!("{:?}", numbers);
}
