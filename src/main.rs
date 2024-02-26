use std::fs::File;
use std::io::{self, Write};
use std::path::{Component, PathBuf};
use std::{env, fs};

fn main() {
    let version_number = "0.0.5";
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
                    println!("cd: There's no path parameter.");
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
            "md" => {
                if parts.len() < 2 {
                    println!("md: There's no name parameter.");
                    continue;
                }
                
                let mut arg = parts.clone();
                arg.remove(0);
                let name = arg.join(" ");

                let new_path = PathBuf::from(name.clone());
                if new_path.components().count() > 1 {
                    println!("md: Only a single directory at a time can be created.");
                    continue;
                }
                
                if new_path.file_name().is_none() || !new_path.starts_with(new_path.file_name().unwrap()) {
                    println!("md: Invalid directory.");
                    continue;
                }

                let new_path = path.join(new_path);

                match fs::create_dir(new_path) {
                    Ok(_) => println!("md: {name} created succesfully."),
                    Err(e) => println!("md: There was an error creating the directory: {}", e)
                }
            },
            "touch" => {
                if parts.len() < 2 {
                    println!("touch: There's no name parameter.");
                    continue;
                }
                
                let mut arg = parts.clone();
                arg.remove(0);
                let name = arg.join(" ");

                let new_path = PathBuf::from(name.clone());
                if new_path.components().count() > 1 {
                    println!("touch: The file name must not contain paths.");
                    continue;
                }
                
                if new_path.file_name().is_none() || !new_path.starts_with(new_path.file_name().unwrap()) {
                    println!("touch: Invalid file name.");
                    continue;
                }

                let new_path = path.join(new_path);

                if new_path.exists() {
                    continue;
                }

                match File::create(new_path) {
                    Ok(_) => println!("touch: File '{name}' created."),
                    Err(e) => println!("touch: An error ocurred while creating the file: {}", e)
                }
            },
            "help" => {
                println!("");
                println!("General commands:");
                println!("cd <dir>  Changes the current directory to the one specified");
                println!("help      Shows the available commands");
                println!("ls        Shows all elements in a directory");
                println!("md        Creates a directory");
                println!("touch     Creates a new file");
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