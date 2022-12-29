use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tente acertar o numero de 0 a 100!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //    println!("O número secreto é: {}", secret_number);

    loop {
        println!("Digite o seu papite.");

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
            Ordering::Less => println!("Chutou baixo!"),
            Ordering::Greater => println!("Chutou alto!"),
            Ordering::Equal => {
                println!("Aew!! Acertou!");
                break;
            }
        }
    }
}
