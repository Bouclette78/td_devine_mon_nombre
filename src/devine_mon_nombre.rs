use std::io;

fn main() {
    println!("Devine mon nombre ! \n\nsaisissez votre proposition");
    
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();

    println!("tu as choisi {}", buffer);
}