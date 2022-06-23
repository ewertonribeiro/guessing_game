use std::io::stdin;

fn main() {
    println!("Advinhe o numero Secreto");

    println!("Qual seu chute?");

    //Cria a variavel que armazena o chute
    let mut chute = String::new();

    stdin().read_line(&mut chute).expect("Erro ao ler o input!");

    println!("Voce chutou {}", chute);
}
