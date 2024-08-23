use std::env;
use std::path::Path;

pub fn handle_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

pub fn handle_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_cd(args: &[&str]) {
    let new_dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].to_string()
    };

    if let Err(e) = env::set_current_dir(Path::new(&new_dir)) {
        eprintln!("Error: {}", e);
    }
}

pub fn handle_unknown(command: &str) {
    eprintln!("Command '{}' not found", command);
}
