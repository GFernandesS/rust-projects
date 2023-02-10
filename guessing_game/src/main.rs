use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let random_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Digite sua tentativa:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Tentativa inválida");

        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("Tentativa inválida!");
                continue;
            },
        };

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
            Ordering::Greater => println!("Você digitou um número maior!"),
            Ordering::Less => println!("Você digitou um número menor"),
        }
    }
}
