use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut ganhou = false;

    println!("Advinhe o numero Secreto");

    for index in 1..=10 {
        if index == 10 {
            println!("Esta e a ultima chance!");
        } else {
            println!("Esta e a chance de numero {}\n", index);
        }

        println!("Qual seu chute?\n");

        //Cria a variavel que armazena o chute
        let mut chute = String::new();
        stdin()
            .read_line(&mut chute)
            .expect("Erro ao ler o input!\n");

        //Usa a funcionalidade de Shadowing para converter o tipo de string para u:8
        let chute: u8 = match chute.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor insira um numero!\n\n");
                continue;
            }
        };

        match chute.cmp(&secret_number) {
            Ordering::Less => println!("Seu chute foi menor!\n"),
            Ordering::Greater => println!("Seu chute foi maior!\n"),
            Ordering::Equal => {
                println!("Você acertou!!\n");
                ganhou = true;
                break;
            }
        }
    }

    if ganhou {
        println!("Parabens pela vitoria");
        println!("O Numero secreto era {}", secret_number);
    } else {
        println!("Voce perdeu!");
        println!("O Numero secreto era {}", secret_number);
    }
}
