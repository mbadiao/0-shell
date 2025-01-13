use std::env;
use std::path::Path;

// Fonction principale qui sera appelée par le main
pub fn cd(args: Vec<String>) {
    if let Err(err) = handle_cd(args) {
        println!("{}", err);
    }
}

// Fonction qui gère la logique du cd
fn handle_cd(args: Vec<String>) -> Result<(), String> {
    if args.len() > 1 {
        return Err("cd: too many arguments".to_string());
    }
    let new_dir = if args.is_empty() {
        // No arguments - go to HOME directory
        env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?
    } else {
        match args[0].as_str() {
            ".." => {
                // Go to previous directory
                let current_dir = env::current_dir()
                    .map_err(|e| format!("cd: error retrieving current directory: {}", e))?;

                current_dir
                    .parent()
                    .ok_or("cd: cannot go up from root directory")?
                    .display()
                    .to_string()
            }
            "~" => {
                // Go to HOME directory
                env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?
            }
            dir => dir.to_string(),
        }
    };

    // Change directory
    let path = Path::new(&new_dir);
    env::set_current_dir(&path).map_err(|e| format!("cd: {}: {}", new_dir, e))?;

    Ok(())
}
