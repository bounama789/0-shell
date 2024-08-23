mod commands;

use colored::*;
use commands::{handle_cd, handle_echo, handle_pwd, handle_unknown};
use std::io::{self, Write};

fn main() {
    // Infinite loop to keep the shell running until 'exit' is typed
    loop {
        let prompt = format!("{}{}", "fake@shell:".cyan().bold(), "~$ ".bright_white());

        print!("{}", prompt);

        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue, // If the user just pressed Enter, continue the loop
        };
        let args: Vec<&str> = parts.clone().collect();

        match command {
            "exit" => break,
            "echo" => handle_echo(&args),
            "pwd" => handle_pwd(),
            "cd" => handle_cd(&args),
            _ => handle_unknown(command),
        }
    }
}
