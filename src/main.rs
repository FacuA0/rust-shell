use std::io::{self, Write};
use std::time::Duration;
use std::thread;

fn main() {
    println!("Rust Terminal");
    println!("@FacuA0\n");

    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();
    
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
    
        let value = value.trim();
        match value {
            "exit" => break,
            _ => ()
        }
    }
    
    println!("Exit");
}
