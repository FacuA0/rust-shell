use std::io::{self, Write};
use std::path::PathBuf;
use std::env;

fn main() {
    let version_number = "0.0.4";
    let mut stdout = io::stdout();
    let mut path = env::current_dir().expect("Working directory couldn't be determined.");

    println!("Rust Shell {version_number}");
    println!("Author: @FacuA0\n");

    loop {
        let prompt = String::from(path.to_str().unwrap()) + "> ";
        
        print!("{prompt}");
        stdout.flush().unwrap();
    
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
    
        let parts: Vec<&str> = value.trim().split(" ").collect();
        let command = parts[0];
        match command {
            "cd" => {
                if parts.len() < 2 {
                    println!("cd: There's no parameter.");
                    continue;
                }
                
                let mut arg = parts.clone();
                arg.remove(0);
                let new_path = arg.join(" ");

                change_directory(&mut path, new_path);
            },
            "ls" => {
                list_elements(&mut path);
            },
            "help" => {
                println!("");
                println!("General commands:");
                println!("help      Prints help information");
                println!("cd <dir>  Changes from a directory to another");
                println!("ls        Shows all elements in a directory");
                println!("version   Shows the version information");
                println!("exit      Exits the program");
                println!("");
            }
            "exit" => break,
            "version" => {
                println!("Rust Shell {version_number}");
                println!("Author: @FacuA0\n");
            },
            "" => (),
            _ => println!("Command '{command}' not found. Type 'help' to show available commands."),
        }
    }
    
    println!("Exit");
}

fn change_directory(path: &mut PathBuf, new_path: String) {
    let moving_path = path.join(PathBuf::from(new_path));
    if !moving_path.exists() || !moving_path.is_dir() {
        println!("cd: Directory doesn't exist.");
        return;
    }

    let final_path = moving_path.canonicalize().unwrap();
    match env::set_current_dir(final_path.clone()) {
        Err(e) => println!("cd: There was an error while changing directories: {}", e),
        _ => {
            *path = final_path;
        }
    };
}

fn list_elements(path: &mut PathBuf) {
    let elements = path.read_dir();
    if elements.is_err() {
        println!("ls: There was an error listing the elements: {}", elements.unwrap_err());
        return;
    }

    println!("\nContents of {:?}:", path.file_name().unwrap());

    for element in elements.unwrap() {
        match element {
            Ok(item) => {
                let file_name = item.file_name().to_str().unwrap().to_owned();
                let metadata = item.metadata().unwrap();
                let file_type = 
                    if metadata.is_dir() {"<dir> "} 
                    else if metadata.is_file() {"<file>"}
                    else if metadata.is_symlink() {"<link>"}
                    else {"<unknown>"};

                let size = if metadata.is_file() {
                    format!("- ({})", format_file_length(metadata.len()))
                } else {"".to_owned()};
                
                println!(" {file_type} {file_name} {size}");
            },
            Err(e) => println!("Error: {}", e)
        }
    }

    println!("");
}

fn format_file_length(length: u64) -> String {
    if length < 1000 {
        format!("{} bytes", length)
    }
    else if length < 1_000_000 {
        format!("{} KB", length / 1000)
    }
    else if length < 1_000_000_000 {
        format!("{} MB", length / 1_000_000)
    }
    else if length < 1_000_000_000_000 {
        format!("{} GB", length / 1_000_000_000)
    }
    else if length < 1_000_000_000_000_000 {
        format!("{} TB", length / 1_000_000_000_000)
    }
    else if length < 1_000_000_000_000_000_000 {
        format!("{} PB", length / 1_000_000_000_000_000)
    }
    else {
        format!("{} EB", length / 1_000_000_000_000_000_000)
    }
}