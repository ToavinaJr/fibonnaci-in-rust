use std::io;
mod fibonacci;

fn main() {
    println!("Bienvenue dans le programme pour calculer le nombre de Fibonacci");
    println!("Veuillez entrer un nombre positif");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Invalid input");
    let value : u32 = value.trim().parse().expect("Conversion with u32");
    println!("Fibonacci({}) = {}", value, fibonacci::fibonacci(value));
}
