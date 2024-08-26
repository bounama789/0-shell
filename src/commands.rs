use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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

pub fn handle_clear() {
    // ANSI escape code to clear the screen and move the cursor to the top left
    print!("{}[2J{}[H", 27 as char, 27 as char);
    std::io::stdout().flush().expect("Failed to flush stdout");
}

pub fn handle_mkdir(args: &[&str]) {
    for dir_name in args {
        match fs::create_dir(dir_name) {
            Ok(_) => (),
            Err(e) => eprintln!("Error creating directory '{}': {}", dir_name, e),
        }
    }
}

pub fn handle_cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("Usage: cat <file_name>");
        return;
    }

    let file_name = args[0];

    match fs::File::open(file_name) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                eprintln!("cat: {}: {}", file_name, e);
                return;
            }
            print!("{}", contents);
        }
        Err(e) => eprintln!("Error opening file '{}': {}", file_name, e),
    }
}

pub fn handle_cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let src = Path::new(args[0]);
    let dst = Path::new(args[1]);

    if src.is_dir() {
        eprintln!("Source is a directory. Use a file path for the source.");
        return;
    }

    if dst.is_dir() {
        // If the destination is a directory, copy the file into this directory
        let dst_path = dst.join(src.file_name().unwrap_or_else(|| "file".as_ref()));
        match fs::copy(src, dst_path) {
            Ok(_) => println!("Copied '{}' to '{}'", args[0], dst.display()),
            Err(e) => eprintln!("Error copying file: {}", e),
        }
    } else {
        // If the destination is not a directory, copy the file to the destination
        match fs::copy(src, dst) {
            Ok(_) => println!("Copied '{}' to '{}'", args[0], args[1]),
            Err(e) => eprintln!("Error copying file: {}", e),
        }
    }
}

pub fn handle_mv(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let src = Path::new(args[0]);
    let dst = Path::new(args[1]);

    // Check if source exists
    if !src.exists() {
        eprintln!("Source path does not exist.");
        return;
    }

    // If destination is a directory, append the source file name to the destination path
    let dst_path = if dst.is_dir() {
        // Construct the new path by joining the source file name with the destination directory
        let file_name = src.file_name().unwrap_or_else(|| OsStr::new("file"));
        dst.join(file_name)
    } else {
        // Destination is not a directory, use the provided path
        dst.to_path_buf()   
    };

    // Attempt to rename (move) the file or directory
    match fs::rename(src, &dst_path) {
        Ok(_) => println!("Moved '{}' to '{}'", args[0], dst_path.display()),
        Err(e) => eprintln!("Error moving file: {}", e),
    }
}

pub fn handle_unknown(command: &str) {
    eprintln!("Command '{}' not found", command);
}
