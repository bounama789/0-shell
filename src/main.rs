mod commands;
mod ls;

use colored::*;
use commands::{
    handle_cat, handle_cd, handle_clear, handle_cp, handle_echo, handle_mkdir, handle_mv, handle_pwd, handle_unknown
};
use ls::handle_ls;
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
            "clear" => handle_clear(),
            "echo" => handle_echo(&args),
            "pwd" => handle_pwd(),
            "cd" => handle_cd(&args),
            "ls" => handle_ls(&args),
            "mkdir" => handle_mkdir(&args),
            "cat" => handle_cat(&args),
            "cp" => handle_cp(&args),
            "mv" => handle_mv(&args),
            _ => handle_unknown(command),
        }
    }
}
