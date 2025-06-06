# 0-Shell Project

## Overview
This is a Rust-based shell project, providing a custom command-line interface with various features.

## Prerequisites
- Rust programming language (latest stable version)
- Cargo package manager

## Installation
1. Clone the repository
2. Navigate to the project directory
3. Run `cargo build` to compile the project
4. Run `cargo run` to start the shell

## Features
- Custom shell implementation with a colorful prompt
- Built-in command support:
  - `cd`: Change directory (with home directory support)
  - `pwd`: Print current working directory
  - `echo`: Print arguments
  - `clear`: Clear the screen
  - `ls`: List directory contents with multiple options
    - `-a`: Show hidden files
    - `-l`: Detailed long-format listing
    - `-F`: Classify file types
  - `mkdir`: Create directories
  - `cat`: Display file contents
  - `cp`: Copy files
  - `mv`: Move/rename files
  - `rm`: Remove files and directories (with recursive option)
- Home directory expansion (supports `~`)
- Error handling for commands
- Extensible architecture

## Development
- Build the project: `cargo build`
- Run tests: `cargo test`
- Run the shell: `cargo run`

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
MIT License

