use std::io::{self, Write};
use std::env;

fn main() {
    println!("Rust Terminal");
    println!("Author: @FacuA0\n");

    let mut stdout = io::stdout();

    loop {
        let path = env::current_dir();
        let mut prompt = String::from("> ");
        if path.is_ok() {
            let path = String::from(path.unwrap().to_str().unwrap());
            prompt = path + &prompt;
        }
        
        print!("{prompt}");
        stdout.flush().unwrap();
    
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
    
        let parts: Vec<&str> = value.trim().split(" ").collect();
        let command = parts[0];
        match command {
            "exit" => break,
            _ => ()
        }
    }
    
    println!("Exit");
}
