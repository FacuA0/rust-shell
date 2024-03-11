use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Component, PathBuf, Prefix};
use std::process::Command;
use std::{env, fs};

fn main() {
    let version_number = "0.0.8";
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
            "cp" => {
                copy(&mut path, args);
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
    //println!("Trying to locate a file to execute.");

    let executable = path.join(PathBuf::from(command));
    if !executable.is_file() {
        return Err(())
    }

    let child = Command::new(executable)
        .args(args)
        .spawn();

    if let Err(error) = child {
        println!("Error invoking {:?}: {}", command, error);
        println!("Type: {:?}", error.kind());
        
        if error.raw_os_error().unwrap() != 193 {
            return Ok(())
        }

        println!("{:?} is not an executable file.", command);
        return Ok(())
    }

    let _exit_status = child.unwrap().wait().unwrap();
    //println!("Exit status: {}", exit_status);

    Ok(())
}

fn execute_command(_path: &mut PathBuf, command: &str, args: Vec<&str>) -> Result<(), ()> {
    //println!("Trying to execute command.");
    let child = Command::new(command)
        .args(args)
        .spawn();

    if let Err(error) = child {
        if error.kind() == ErrorKind::NotFound {
            return Err(());
        }

        println!("Error invoking {}: {}", command, error);
        //println!("Type: {:?}", error.kind());
        return Ok(())
    }
    
    let _exit_status = child.unwrap().wait().unwrap();
    //println!("Exit status: {}", exit_status);

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

fn copy(path: &mut PathBuf, args: Vec<&str>) {
    if args.len() < 2 {
        if args.len() == 0 {
            println!("cp: There are no parameters.");
        }
        else {
            println!("cp: Not enough parameters.");
        }
        return;
    }

    let mut source_path = PathBuf::from(args[0]);
    let mut destination_path = PathBuf::from(args[1]);

    if source_path.is_relative() {
        source_path = path.join(source_path);
    }

    if destination_path.is_relative() {
        destination_path = path.join(destination_path);
    }

    if !destination_path.is_dir() {
        if destination_path.exists() {
            println!("cp: The destination path must be a directory.");
        }
        else {
            println!("cp: The destination path doesn't exist.");
        }
        return;
    }
    
    if !source_path.exists() {
        println!("cp: The source doesn't exist.");
        return;
    }

    let source_name = source_path.file_name().unwrap();

    if source_path.is_file() {
        let destination_name = destination_path.join(source_name);

        if source_path == destination_name {
            println!("cd: It's not possible to copy this file on the same location.");
            return;
        }

        if destination_name.exists() {
            println!("cd: There's a file with the same name on the destination.");
            return;
        }

        match fs::copy(source_path, destination_name) {
            Ok(_) => println!("cp: 1 file copied succesfully."),
            Err(error) => println!("There was an error copying the file: {error}")
        }
    }
    else {
        match copy_element(&source_path, &destination_path) {
            Ok(amount) => println!("{amount} elements were copied successfully"),
            Err(error) => println!("There was an error: {error}")
        }
    }
}

fn copy_element(source: &PathBuf, destination: &PathBuf) -> std::io::Result<i32> {
    let source_name = source.file_name().unwrap();
    let destination_name = destination.join(source_name);
    let mut count = 0;
    
    if source.is_file() {
        if *source == destination_name {
            let err = format!("{}: Copy on the same location", source.display());
            return Err(Error::new(ErrorKind::Other, err));
        }
        
        if destination_name.exists() {
            let err = format!("{}: Destination already exists", destination_name.display());
            return Err(Error::new(ErrorKind::AlreadyExists, err));
        }

        fs::copy(source, destination_name)?;

        count = 1;
    }
    else if source.is_dir() {
        if !destination_name.exists() {
            fs::create_dir(&destination_name)?;
        }

        for element in source.read_dir()? {
            let element = element?;
            count += copy_element(&element.path(), &destination_name)?;
        }
    }

    Ok(count)
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