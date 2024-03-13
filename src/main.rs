use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Component, PathBuf, Prefix};
use std::process::Command;
use std::{env, fs};

fn main() {
    let version_number = "v0.0.9";
    let mut stdout = io::stdout();
    let mut path = env::current_dir().expect("Working directory couldn't be determined.");

    let ftitle = "\x1B[1;31m";
    let fversion = "\x1B[33m";
    let fauthor = "\x1B[36m";
    let freset = "\x1B[0m";

    println!("{ftitle}Rust Shell{freset} {fversion}{version_number}{freset}");
    println!("Created by {fauthor}@FacuA0{freset}\n");

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
            "mv" => {
                move_files(&mut path, args);
            },
            "cp" => {
                copy_files(&mut path, args);
            },
            "help" => {
                help_command(args);
            }
            "version" => {
                println!("{ftitle}Rust Shell{freset} {fversion}{version_number}{freset}");
                println!("Created by {fauthor}@FacuA0{freset}\n");
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

fn move_files(path: &mut PathBuf, args: Vec<&str>) {
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
            Ok(_) => println!("cp: File copied succesfully."),
            Err(error) => println!("There was an error copying the file: {error}")
        }
    }
    else {
        match move_element(&source_path, &destination_path) {
            Ok(amount) => println!("{amount} elements were copied successfully"),
            Err(error) => println!("There was an error: {error}")
        }
    }
}

fn move_element(source: &PathBuf, destination: &PathBuf) -> std::io::Result<i32> {
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

fn copy_files(path: &mut PathBuf, args: Vec<&str>) {
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
            Ok(_) => println!("cp: File copied succesfully."),
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
    if let Err(error) = elements {
        println!("ls: There was an error listing the elements: {}", error);
        return;
    }

    let mut files: Vec<fs::DirEntry> = vec![];
    let mut directories: Vec<fs::DirEntry> = vec![];
    let mut errors = 0;

    for element in elements.unwrap() {
        match element {
            Ok(item) => {
                let metadata = item.path().metadata();
                if let Ok(metadata) = metadata {
                    if metadata.is_file() {
                        files.push(item);
                    }
                    else if metadata.is_dir() {
                        directories.push(item);
                    }

                    continue;
                }

                errors += 1;
            },
            Err(_) => errors += 1
        }
    }


    let location_name = match path.file_name() {
        Some(name) => name,
        None => path.as_os_str()
    };

    let error_text = match errors {
        0 => "".to_owned(),
        _ => format!(" (including {errors} errors)")
    };

    println!("\nContents of {:?}{}:", location_name, error_text);


    for directory in directories {
        let file_name = directory.file_name().to_str().unwrap().to_owned();
        let metadata = directory.metadata().unwrap();
        let file_type = 
            if metadata.is_symlink() {"<dir link>"} 
            else {"<dir>"};
        
        println!(" {file_type:11} {file_name}");
    }

    for file in files {
        let file_name = file.file_name().to_str().unwrap().to_owned();
        let metadata = file.metadata().unwrap();
        let file_type = 
            if metadata.is_symlink() {"<file link>"}
            else {"<file>"};

        let size = format_file_length(metadata.len());
        
        println!(" {file_type:11} {file_name} - ({size})");
    }

    println!("");
}

fn format_file_length(length: u64) -> String {
    let number = if length >= 1000 {
        let float = length as f64;
        let log = float.log10();
        
        let float = float / 10f64.powi((log as i32) / 3 * 3);
        let truncate = 2 - (log as u32) % 3;

        format!("{:.1$}", float, truncate as usize)
    }
    else { format!("{length}") };

    if length < 1000 {
        format!("{} B", number)
    }
    else if length < 1_000_000 {
        format!("{} KB", number)
    }
    else if length < 1_000_000_000 {
        format!("{} MB", number)
    }
    else if length < 1_000_000_000_000 {
        format!("{} GB", number)
    }
    else if length < 1_000_000_000_000_000 {
        format!("{} TB", number)
    }
    else if length < 1_000_000_000_000_000_000 {
        format!("{} PB", number)
    }
    else {
        format!("{} EB", number)
    }
}

fn help_command(args: Vec<&str>) {
    if args.len() == 0 {
        println!("");
        println!("General commands:");
        println!("cd            Changes the current directory to the one specified");
        println!("cp            Copies an element to another location");
        println!("help          Shows the available commands");
        println!("ls            Shows all elements in a directory");
        println!("md            Creates a directory");
        println!("mv            Moves an element to another location");
        println!("touch         Creates a new file");
        println!("rm            Removes an element");
        println!("version       Shows the version information");
        println!("exit          Exits the shell");
        println!("");

        return;
    }

    let command = args[0];
    match command {
        "cd" => {
            println!("Command: cd <directory>");
            println!("Description: Changes the current working directory to the one specified on the argument.");
            println!("");
            println!("Arguments:");
            println!(" - <directory>      The directory where the shell should change.");
            println!("");
        },
        "cp" => {
            println!("Command: cp [-y] [-n] [-r] <source> <destination>");
            println!("Description: Copies a specific source element (either a file or a directory) into a destination directory.");
            println!("If a directory is specified as a source, it copies it along with its contents. Any existing directory on destination will receive the contents of the directory being copied.");
            println!("The default behavior when a file is duplicated is to ask the user if it should be replaced, cancelled or renamed.");
            println!("When the -y flag is used, the command will replace any destination file by default unless the -r flag is used.");
            println!("");
            println!("Arguments:");
            println!(" - <source>         The source element to be copied.");
            println!(" - <destination>    The destination directory.");
            println!(" - [-y]             A flag that makes the operation to continue even if there are duplicate elements.");
            println!(" - [-n]             A flag that cancels the entire operation if a single element is duplicated.");
            println!(" - [-r]             A flag that indicates that, if an element is duplicated, it should be numbered to avoid conflicts.");
            println!("");
        },
        "help" => {
            println!("Command: help [command]");
            println!("Description: Shows the available commands when invoked without arguments.");
            println!("With an argument, it shows the description of a specific built-in shell command.");
            println!("");
            println!("Arguments:");
            println!(" - [command]        A command to be described.");
            println!("");
        },
        "ls" => {
            println!("Command: ls");
            println!("Description: Lists the files and directories in the current location.");
            println!("");
        },
        "md" => {
            println!("Command: md <directory>");
            println!("Description: Creates a new directory with the specified name.");
            println!("");
            println!("Arguments:");
            println!(" - <directory>      The directory name to be used.");
            println!("");
        },
        "mv" => {
            println!("Command: mv [-y] [-n] [-r] <source> <destination>");
            println!("Description: Moves a specific source element (either a file or a directory) into a destination directory, removing any original file in the former location.");
            println!("If a directory is specified as a source, it moves it along with its contents. Any existing directory on destination will receive the contents of the directory being moved.");
            println!("The default behavior when a file is duplicated is to ask the user if it should be replaced, cancelled or renamed.");
            println!("When the -y flag is used, the command will replace any destination file by default unless the -r flag is used.");
            println!("");
            println!("Arguments:");
            println!(" - <source>         The source element to be moved.");
            println!(" - <destination>    The destination directory.");
            println!(" - [-y]             A flag that makes the operation to continue even if there are duplicate elements.");
            println!(" - [-n]             A flag that cancels the entire operation if a single element is duplicated.");
            println!(" - [-r]             A flag that indicates that, if an element is duplicated, it should be numbered to avoid conflicts.");
            println!("");
        },
        "touch" => {
            println!("Command: touch <file>");
            println!("Description: Creates a new empty file with the specified name.");
            println!("");
            println!("Arguments:");
            println!(" - <file>           The file name to be used.");
            println!("");
        },
        "rm" => {
            println!("Command: rm [-r] <element>");
            println!("Description: Removes the file or directory at the specified location.");
            println!("If a directory has inner elements, it won't be removed unless the -r flag was used.");
            println!("");
            println!("Arguments:");
            println!(" - <element>        The file or directory to be removed.");
            println!(" - [-r]             A flag that removes a directory recursively, which includes any internal files and directories in it.");
            println!("");
        }
        "version" => {
            println!("Command: version");
            println!("Description: Prints the current version and author of the shell.");
            println!("");
        },
        "exit" => {
            println!("Command: exit");
            println!("Description: Exits the shell.");
            println!("");
        },
        _ => {
            println!("help: There's no built-in command named '{command}'. Type 'help' to show available commands.");
        }
    };
}