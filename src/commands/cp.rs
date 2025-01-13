use std::fs;
use std::path::Path;
use std::io;

pub fn cp(args: Vec<String>) {
    if args.len() != 2 {
        println!("cp: missing file operand");
        println!("Usage: cp <source> <destination>");
        return;
    }
    let source = &args[0];
    let destination = &args[1];
    if let Err(err) = copy(&source, &destination) {
        println!("cp: {}", err);
    }
}

fn copy(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // Vérifier si la source existe
    if !source_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot stat '{}': No such file or directory", source)
        ));
    }

    // Vérifier si la source est un répertoire
    if source_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is a directory (not copied)", source)
        ));
    }

    // Déterminer le chemin final de destination
    let final_destination = if destination_path.is_dir() {
        destination_path.join(source_path.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid source filename")
        })?)
    } else {
        destination_path.to_path_buf()
    };

    // Vérifier si le chemin de destination existe
    if !final_destination.parent().map_or(true, |p| p.exists()) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot found '{}': No such directory", destination)
        ));
    }

    // Copier le fichier
    fs::copy(source_path, &final_destination).map(|_| ())
}