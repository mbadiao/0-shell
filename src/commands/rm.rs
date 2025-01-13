use std::fs;
use std::path::Path;
use std::io;

pub fn rm(args: Vec<String>) {
    if args.is_empty() {
        println!("rm: missing operand");
        println!("Usage: rm [-r] <file1> [file2 ...]");
        return;
    }

    let mut is_recursive = false;
    let mut files = Vec::new();

    // Loop through arguments to identify flags and files
    for arg in args {
        if &arg == "-r" {
            is_recursive = true;
        } else {
            files.push(arg);
        }
    }
    
    if files.is_empty() {
        println!("rm: missing operand");
        println!("Usage: rm [-r] <file1> [file2 ...]");
        return;
    }
    
    for file in files {
        if let Err(err) = remove_file(&file, is_recursive) {
            println!("rm: {}", err);
        }
    }
}

fn remove_file(file: &str, recursive: bool) -> io::Result<()> {
    let path = Path::new(file);
    // Check if file exists
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot remove '{}': No such file or directory", file)
        ));
    }
    // If it's a directory
    if path.is_dir() {
        if recursive {
            // With -r, remove the directory and its contents
            remove_dir_recursive(path)
        } else {
            // Without -r, return an error
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("cannot remove '{}': Is a directory", file)
            ))
        }
    } else {
        // Remove a normal file
        fs::remove_file(path)
    }
}

fn remove_dir_recursive(path: &Path) -> io::Result<()> {
    // If it's a directory, we must first remove its contents
    if path.is_dir() {
        // Loop through directory contents
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursive call for subdirectories
                remove_dir_recursive(&path)?;
            } else {
                // Remove files
                fs::remove_file(&path)?;
            }
        }
        // Once the directory is empty, we can remove it
        fs::remove_dir(path)
    } else {
        // If it's not a directory, remove it directly
        fs::remove_file(path)
    }
}