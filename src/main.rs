use std::env;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    loop {
        // Display the prompt
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read the user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim the input to remove whitespace and newline characters
        let input = input.trim();

        // Split the input into the command and arguments
        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue, // If the user just pressed Enter, continue the loop
        };
        let args: Vec<&str> = parts.collect();

        // Handle the 'exit' command
        if command == "exit" {
            break;
        }
        // Handle the 'echo' command
        else if command == "echo" {
            println!("{}", args.join(" "));
        }
        // Handle the 'pwd' command
        else if command == "pwd" {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        // Handle the 'cd' command
        else if command == "cd" {
            let new_dir = if args.is_empty() {
                // If no argument is given, default to the home directory
                env::var("HOME").unwrap_or_else(|_| String::from("/"))
            } else {
                args[0].to_string()
            };

            if let Err(e) = env::set_current_dir(Path::new(&new_dir)) {
                eprintln!("Error: {}", e);
            }
        }
        // If the command is not recognized
        else {
            eprintln!("Command '{}' not found", command);
        }
    }
}
