mod commands;
use commands::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm};
use std::{
    env,
    io::{self, Write},
};

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";

fn main() {
    loop {
        let path = match env::current_dir() {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(e) => {
                eprintln!("Error getting the current directory: {}", e);
                break;
            }
        };

        print!("{}{}{}$ ", CYAN, path, RESET);
        std::io::stdout().flush().unwrap();

        // wait for a command
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.is_empty() {
            break;
        }

        let trimmed = input.trim();

        let parts = get_parts(trimmed.to_string());
        let (command, args) = parts;

        match command.as_str() {
            "" => continue,
            "exit" => break,
            "echo" => echo(args),
            "cd" => cd(args),
            "ls" => ls(args),
            "pwd" => pwd(),
            "cat" => cat(args),
            "cp" => cp(args),
            "rm" => rm(args),
            "mv" => mv(args),
            "mkdir" => mkdir(args),
            _ => println!("Command '{}' not found.", command),
        }
    }
}

fn get_parts(input: String) -> (String, Vec<String>) {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_double_quotes = false;
    let mut in_single_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                if !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                    if !in_double_quotes && !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(c);
                }
            }
            '\'' => {
                if !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                    if !in_single_quotes && !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(c);
                }
            }
            ' ' if !in_double_quotes && !in_single_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    if let Some((command, args)) = parts.split_first() {
        (
            command.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        )
    } else {
        ("".to_string(), vec![])
    }
}
