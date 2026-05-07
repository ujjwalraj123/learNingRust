use std::{ cmp::Ordering, io };
use rand::Rng;
fn main() {
    println!("Guessing games begins");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please enter your guessed number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read Input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!("Your guessed Number:{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try a greater Number"),
            Ordering::Equal => {
                println!("Congratulations your guessed Number has matched");
                break;
            }
            Ordering::Greater => println!("Try a Smaller"),
        }
    }
}
