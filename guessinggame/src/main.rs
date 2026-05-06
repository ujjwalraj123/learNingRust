use std::io;

fn main() {
    println!("Guessing games begins");
    println!("Please enter your guessed number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read Input");
    println!("Your guessed Number:{guess}");
}
