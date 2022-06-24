use rand::{self, Rng};
use std::{cmp::Ordering, io::stdin};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Advinhe o numero Secreto");
    println!("Qual seu chute?");

    //Cria a variavel que armazena o chute
    let mut chute = String::new();
    stdin().read_line(&mut chute).expect("Erro ao ler o input!");

    //Usa a funcionalidade de Shadowing para converter o tipo de string para i:8
    let chute: u8 = chute.trim().parse().expect("Por favor digite um numero!");

    match chute.cmp(&secret_number) {
        Ordering::Less => println!("Seu chute foi menor!"),
        Ordering::Greater => println!("Seu chute foi maior!"),
        Ordering::Equal => println!("Você acertou!!"),
    }

    println!("O Numero secreto era {}", secret_number);
    println!("Voce chutou {}", chute);
}
