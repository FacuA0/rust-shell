use std::fs::File;
use std::io::{self, Write};
use std::path::{Component, PathBuf, Prefix};
use std::process::Command;
use std::{env, fs, time};

fn main() {
    let version_number = "0.0.7";
    let mut stdout = io::stdout();
    let mut path = env::current_dir().expect("Working directory couldn't be determined.");

    println!("Rust Shell {version_number}");
    println!("Created by @FacuA0\n");

    loop {
        let prompt = String::from(path.to_str().unwrap()) + "> ";
        
        print!("{prompt}");
        stdout.flush().unwrap();
    
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
    
        let raw_parts: Vec<&str> = value.trim().split(" ").collect();
        let mut parts: Vec<String> = vec![];

        let mut is_string = false;
        let mut sum_string = String::new();

        for part in raw_parts {
            if part.chars().next() == Some('"') {
                is_string = true;
            }
            
            if is_string {
                sum_string.push(' ');
                sum_string.push_str(part);
            }
            else {
                parts.push(part.to_string());
            }
            
            if part.chars().last() == Some('"') {
                if is_string {
                    parts.push(sum_string[1..].to_string());
                }

                is_string = false;
                sum_string.clear();
            }
        }

        if is_string {
            println!("Invalid syntax: Double quotes not closed.");
            continue;
        }

        let parts = {
            let mut str_parts: Vec<&str> = vec![];
            for str in parts.iter() {
                str_parts.push(str.as_str());
            }

            str_parts
        };

        let command = parts[0];
        let mut args = parts.clone();
        args.remove(0);

        match command {
            "cd" => {
                change_directory(&mut path, args);
            },
            "ls" => {
                list_elements(&mut path);
            },
            "md" => {
                make_directory(&mut path, args);
            },
            "touch" => {
                create_file(&mut path, args);
            },
            "rm" => {
                remove_element(&mut path, args);
            },
            "help" => {
                println!("");
                println!("General commands:");
                println!("cd <dir>       Changes the current directory to the one specified");
                println!("help           Shows the available commands");
                println!("ls             Shows all elements in a directory");
                println!("md <dir>       Creates a directory");
                println!("touch <file>   Creates a new file");
                println!("rm [-r] <dir>  Removes an element");
                println!("version        Shows the version information");
                println!("exit           Exits the shell");
                println!("");
            }
            "version" => {
                println!("Rust Shell {version_number}");
                println!("Created by @FacuA0\n");
            },
            "exit" => break,
            "" => (),
            _ => {
                if execute_command(&mut path, command, args.clone()).is_ok() { continue }
                if execute_local_file(&mut path, command, args.clone()).is_ok() { continue }

                println!("Command '{command}' not found. Type 'help' to show available commands.")
            },
        }
    }
    
    println!("Exit");
}

fn execute_local_file(path: &mut PathBuf, command: &str, args: Vec<&str>) -> Result<(), ()> {
    println!("Trying to locate a file to execute.");
    let elements = path.read_dir();
    if elements.is_err() {
        return Err(())
    }

    let t1 = time::Instant::now();
    for element in elements.unwrap() {
        if element.is_err() { continue }
        
        let element = element.unwrap();
        if element.metadata().is_err() { continue }
        
        let metadata = element.metadata().unwrap();
        if !metadata.is_file() { continue }
        
        let file_name = element.file_name();
        if file_name != command { continue }

        let child = Command::new(element.path())
            .args(args)
            .spawn();

        if let Err(error) = child {
            println!("Error invoking {:?}: {}", file_name, error);
            println!("Type: {:?}", error.kind());
            
            if error.raw_os_error().unwrap() != 193 {
                return Ok(())
            }
            
            if let Some(error) = error.get_ref() {
                println!("Inner error: {}", error);
            }
            else {
                println!("No inner errors.");
            }

            println!("{:?} is not an executable file.", file_name);
            return Ok(())
        }
        
        let exit_status = child.unwrap().wait().unwrap();
        println!("Exit status: {}", exit_status);
        
        return Ok(())
    }
    
    let t2 = time::Instant::now();
    println!("Time: {:?}", t2.duration_since(t1));
    
    Err(())
}

fn execute_command(_path: &mut PathBuf, command: &str, args: Vec<&str>) -> Result<(), ()> {
    println!("Trying to execute command.");
    let child = Command::new(command)
        .args(args)
        .spawn();

    if let Err(error) = child {
        println!("Error invoking {}: {}", command, error);
        println!("Type: {:?}", error.kind());
        return Err(())
    }
    
    let exit_status = child.unwrap().wait().unwrap();
    println!("Exit status: {}", exit_status);

    Ok(())
}

fn make_directory(path: &mut PathBuf, args: Vec<&str>) {
    if args.len() < 1 {
        println!("md: There's no name parameter.");
        return;
    }
    
    let name = args.join(" ");

    let new_path = PathBuf::from(name.clone());
    if new_path.components().count() > 1 {
        println!("md: Only a single directory can be created at a time.");
        return;
    }
    
    if new_path.file_name().is_none() || !new_path.starts_with(new_path.file_name().unwrap()) {
        println!("md: Invalid directory.");
        return;
    }

    let new_path = path.join(new_path);

    match fs::create_dir(new_path) {
        Ok(_) => (),
        Err(e) => println!("md: There was an error creating the directory: {}", e)
    }
}

fn create_file(path: &mut PathBuf, args: Vec<&str>) {
    if args.len() < 1 {
        println!("touch: There's no name parameter.");
        return;
    }
    
    let name = args.join(" ");

    let new_path = PathBuf::from(name.clone());
    if new_path.components().count() > 1 {
        println!("touch: The file name must not contain paths.");
        return;
    }
    
    let file_name = new_path.file_name();
    if file_name.is_none() || !new_path.starts_with(file_name.unwrap()) {
        println!("touch: Invalid file name.");
        return;
    }

    let new_path = path.join(new_path);
    if new_path.exists() {
        return;
    }

    match File::create(new_path) {
        Ok(_) => (),
        Err(e) => println!("touch: An error ocurred while creating the file: {}", e)
    }
}

fn remove_element(path: &mut PathBuf, args: Vec<&str>) {
    if args.len() < 1 {
        println!("rm: There are no parameters.");
        return;
    }

    let mut recursive = false;
    let mut dest_path: Vec<&str> = vec![];
    
    let mut flags = true;
    for arg in args {
        if flags {
            let go_next = match arg {
                "-r" => { recursive = true; true },
                _ => { flags = false; false },
            };

            if go_next {continue};
        }

        dest_path.push(arg);
    }

    // Still in flags mode - never got a path
    if flags {
        println!("rm: There's no path parameter.");
        return;
    }

    let dest_path = path.join(PathBuf::from(dest_path.join(" ")));

    if path.starts_with(&dest_path) {
        println!("rm: The current working directory is inside of the one being removed.");
        return;
    }
    
    if !path.exists() {
        println!("rm: The location doesn't exist.");
        return;
    }

    if dest_path.is_file() {
        match fs::remove_file(dest_path) {
            Ok(_) => (),
            Err(e) => println!("rm: The file couldn't be removed: {}", e)
        }
        return;
    }

    if recursive {
        match fs::remove_dir_all(dest_path) {
            Ok(_) => (),
            Err(e) => println!("rm: The tree couldn't be removed: {}", e)
        }
    }
    else {
        match fs::remove_dir(dest_path) {
            Ok(_) => (),
            Err(e) => println!("rm: The directory couldn't be removed: {}", e)
        }
    }
}

fn change_directory(path: &mut PathBuf, args: Vec<&str>) {
    if args.len() < 1 {
        println!("cd: There's no path parameter.");
        return;
    }
    
    let new_path = args.join(" ");
    let moving_path = path.join(PathBuf::from(new_path));
    if !moving_path.exists() || !moving_path.is_dir() {
        println!("cd: Directory doesn't exist.");
        return;
    }

    let canonical = moving_path.canonicalize().unwrap();
    let mut final_path = PathBuf::new();

    // Removing verbatim (extended) prefix from Windows paths (\\?\)
    for component in canonical.components() {
        match component {
            Component::Prefix(prefix) => {
                let element = match prefix.kind() {
                    Prefix::Verbatim(name) => name.to_str().unwrap().to_owned(),
                    Prefix::VerbatimUNC(server, folder) => format!("\\\\{}\\{}", server.to_str().unwrap(), folder.to_str().unwrap()),
                    Prefix::VerbatimDisk(disk) =>  String::from(char::from(disk)) + ":",
                    _ => component.as_os_str().to_str().unwrap().to_owned()
                };

                final_path.push(element);
            },
            _ => final_path.push(component)
        }
    }

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