use rand::{self, Rng};
use std::io::stdin;

fn main() {
    println!("Advinhe o numero Secreto");

    println!("Qual seu chute?");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //Cria a variavel que armazena o chute
    let mut chute = String::new();

    stdin().read_line(&mut chute).expect("Erro ao ler o input!");

    println!("O Numero secreto era {}", secret_number);
    println!("Voce chutou {}", chute);
}
