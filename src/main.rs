use std::io::{self, Write};
use std::env;
use enable_ansi_support as ansi;

mod commands;

fn main() {
    let version_number = "v0.0.9";
    let mut stdout = io::stdout();
    let mut path = env::current_dir().expect("Working directory couldn't be determined.");

    let mut ftitle = "\x1B[1;31m";
    let mut fversion = "\x1B[33m";
    let mut fauthor = "\x1B[36m";
    let mut freset = "\x1B[0m";

    match ansi::enable_ansi_support() {
        Err(_) => {
            ftitle = "";
            fversion = "";
            fauthor = "";
            freset = "";
        },
        Ok(_) => ()
    }

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
                commands::change_directory(&mut path, args);
            },
            "ls" => {
                commands::list_elements(&mut path);
            },
            "md" => {
                commands::make_directory(&mut path, args);
            },
            "touch" => {
                commands::create_file(&mut path, args);
            },
            "rm" => {
                commands::remove_element(&mut path, args);
            },
            "mv" => {
                commands::move_files(&mut path, args);
            },
            "cp" => {
                commands::copy_files(&mut path, args);
            },
            "help" => {
                commands::help_command(args);
            }
            "version" => {
                println!("{ftitle}Rust Shell{freset} {fversion}{version_number}{freset}");
                println!("Created by {fauthor}@FacuA0{freset}\n");
            },
            "exit" => break,
            "" => (),
            _ => {
                if commands::execute_command(&mut path, command, args.clone()).is_ok() { continue }
                if commands::execute_local_file(&mut path, command, args.clone()).is_ok() { continue }
                
                println!("Command '{command}' not found. Type 'help' to show available commands.")
            },
        }
    }
    
    println!("Exit");
}