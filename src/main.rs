use std::io::{self, Write};
use std::time::Duration;
use std::thread;

fn main() {
    println!("Hola!");
    print!("Por favor indique su edad: ");
    io::stdout().flush().unwrap();

    let mut valor = String::new();
    io::stdin().read_line(&mut valor).unwrap();

    print!("Calculando.");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    io::stdout().flush().unwrap();
    print!(".");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    print!(".");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("");

    let edad = valor.trim().parse::<i32>();
    match edad {
        Ok(edad) => println!("Tu edad es {edad}!"),
        Err(_) => println!("Edad inválida. Vuelva a hacerlo.")
    }
}
