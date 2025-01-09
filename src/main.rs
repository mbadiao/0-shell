pub mod commands;

use commands::{
    cat::cat, cd::cd, cp::cp, echo::echo, ls::ls, mkdir::mkdir, mv::mv, pwd::pwd, rm::rm,
};

use std::io::Write;

fn main() {
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        // wait for a command
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("Error reading input.");
            continue;
        }

        let trimmed = input.trim();

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let (command, args) = parts.split_first().unwrap_or((&"", &[]));

        match *command {
            "" => continue,
            "exit" => break,
            "echo" => echo(args),
            "cd" => cd(args),
            "ls" => ls(args),
            "pwd" => pwd(args),
            "cat" => cat(args),
            "cp" => cp(args),
            "rm" => rm(args),
            "mv" => mv(args),
            "mkdir" => mkdir(args),
            _ => println!("Command '{}' not found.", command),
        }
    }
}
