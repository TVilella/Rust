use std::io;
use rand::Rng;

fn main() {
    println!("Pense em um número!");

    let secret_number = rand::thread_rng()
				.gen_range(1..101);

    println!("Por favor digite o número que pensou.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    println!("Voce pensou no número:... {}", guess);
    println!("O número secreto é: {}", secret_number);
}
