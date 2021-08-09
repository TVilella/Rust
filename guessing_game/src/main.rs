use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pense em um número!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("O número secreto é: {}", secret_number);

    println!("Por favor digite o número que pensou.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    let guess: u32 = guess
                        .trim()
                        .parse()
                        .expect("Por favor, digite um número!");

    println!("Voce pensou no número:... {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Muito pequeno!"),
        Ordering::Greater => println!("Muito grande!"),
        Ordering::Equal => println!("Aew!! Acertou!"),
    }
}
