mod commands;
mod ls;

use colored::*;
use commands::{
    handle_cat, handle_cd, handle_clear, handle_cp, handle_echo, handle_mkdir, handle_mv, handle_pwd, handle_unknown,handle_rm
};
use ls::handle_ls;
use std::{env, io::{self, Write}};


fn main() {
    // Infinite loop to keep the shell running until 'exit' is typed
    
    loop {
        // let prompt = format!("{}{}", "fake@shell:".cyan().bold(), "~$ ".bright_white());
        let current_dir = match env::current_dir() {
            Ok(path) => path.display().to_string(),
            Err(_) => "Unknown".to_string(), // Gestion des erreurs en cas d'échec
        };

        let prompt_path = if let Some(home_dir) = get_home_directory() {
            current_dir.replace(&home_dir, "~")
        } else {
            current_dir.clone()
        };

        let prompt = format!("{}{}{}","fake@shell:".green().bold(), prompt_path.cyan().bold(),"$ ".bright_white());

        print!("{}", prompt);

        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Some(home_dir) = get_home_directory() {
             input=input.replace("~", &home_dir);
        }
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
            "rm" => handle_rm(&args),
            _ => handle_unknown(command),
        }
    }
}

fn get_home_directory() -> Option<String> {
    env::var("HOME").ok()
}
