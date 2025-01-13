mod commands;
use commands::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm};
use std::{env, io::{self, Write}};

fn main() {
    loop {
        let path = match env::current_dir() {
            Ok(path) => {
                path
            }
            Err(e) => {
                eprintln!("Error getting the current directory: {}", e);
                break;
            }
        };

        print!("{:?}$ ", path.to_string_lossy());
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
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                // Basculer l'état d'analyse des guillemets
                in_quotes = !in_quotes;
                if !in_quotes && !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            ' ' if !in_quotes => {
                // Ajouter la partie actuelle si un espace est trouvé en dehors des guillemets
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                // Ajouter le caractère courant à la partie actuelle
                current.push(c);
            }
        }
    }

    // Ajouter la dernière partie si elle existe
    if !current.is_empty() {
        parts.push(current);
    }

    // Séparer la commande (première partie) et les arguments
    if let Some((command, args)) = parts.split_first() {
        (
            command.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        )
    } else {
        // Si aucune commande ou argument, renvoyer une valeur par défaut
        ("".to_string(), vec![])
    }
}
