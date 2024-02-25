use std::io::{self, Write};
use std::path::PathBuf;
use std::env;

fn main() {
    let version_number = "0.0.3";
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
                let elements = path.read_dir();
                if elements.is_err() {
                    println!("ls: There was an error listing the elements: {}", elements.unwrap_err());
                    continue;
                }

                println!("Contents of {:?}:", path.file_name().unwrap());

                for element in elements.unwrap() {
                    match element {
                        Ok(item) => {
                            let file_name = item.file_name().to_str().unwrap().to_owned();
                            let file_type = item.file_type().unwrap();
                            if file_type.is_dir() {
                                println!("{file_name} <dir>");
                            }
                            else if file_type.is_file() {
                                println!("{file_name} <file>");
                            }
                            else if file_type.is_symlink() {
                                println!("{file_name} <link>");
                            }
                            else {
                                println!("{file_name} <unknown>");
                            }
                        },
                        Err(e) => println!("Error: {}", e)
                    }
                }
            },
            "exit" => break,
            "version" => {
                println!("Rust Shell {version_number}");
                println!("Author: @FacuA0\n");
            },
            "" => (),
            _ => println!("Command '{command}' not found."),
        }
    }
    
    println!("Exit");
}

fn change_directory(path: &mut PathBuf, new_path: String) {
    if new_path == "." {
        return;
    }

    if new_path == ".." {
        let parent_path = path.parent();
        if parent_path.is_none() {
            return;
        }

        *path = parent_path.unwrap().to_path_buf();
        match env::set_current_dir(path.clone()) {
            Err(e) => println!("There was an error while changing directories: {}", e),
            _ => return
        };
    }

    let moving_path = path.join(PathBuf::from(new_path));
    if !moving_path.exists() || !moving_path.is_dir() {
        println!("cd: Directory doesn't exist.");
        return;
    }

    *path = moving_path;
    match env::set_current_dir(path.clone()) {
        Err(e) => println!("cd: There was an error while changing directories: {}", e),
        _ => ()
    };
}