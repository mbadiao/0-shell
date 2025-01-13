use std::{fs, path::Path};

pub fn mkdir(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    let mut recursive = false;
    let mut paths:Vec<String> = Vec::new();

    // Handle the flag -p
    for arg in args {
        if arg == "-p" {
            recursive = true;
        } else {
            paths.push(arg);
        }
    }

    for path in paths {
        let path = Path::new(&path);
        let result = if recursive {
            fs::create_dir_all(path) // Create nested directories
        } else {
            fs::create_dir(path) // Create a single directory
        };

        match result {
            Ok(_) => {}
            Err(e) => eprintln!("mkdir: cannot create directory '{}': {}", path.display(), e),
        }
    }
}
