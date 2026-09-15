use std::io;
use rand::Rng;

fn main() {
    println!("Devine mon nombre ! \n\nsaisissez votre proposition");
    
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();

    println!("tu as choisi {}", buffer.trim());

    let rdm = rand::thread_rng().gen_range(1..101);
    println!("nb aléatoire {}", rdm);
}