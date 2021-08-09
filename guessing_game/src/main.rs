use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pense em um número de 0 a 100!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //    println!("O número secreto é: {}", secret_number);

    loop {
        println!("Digite o número que pensou.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número válido!!!");
                continue;
            }
        };

        //        println!("Voce escolheu o número:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Aew!! Acertou!");
                break;
            }
        }
    }
}
