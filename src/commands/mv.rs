use std::fs;
use std::path::Path;
use std::io;

pub fn mv(args: &[&str]) {
    if args.len() != 2 {
        println!("mv: missing file operand");
        println!("Usage: mv <source> <destination>");
        return;
    }
    let source = args[0];
    let destination = args[1];
    if let Err(err) = move_file(source, destination) {
        println!("mv: {}", err);
    }
}

fn move_file(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // Vérifier si la source existe
    if !source_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot stat '{}': No such file or directory", source)
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

    // Vérifier si le répertoire parent de la destination existe
    if !final_destination.parent().map_or(true, |p| p.exists()) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot move '{}': No such directory", destination)
        ));
    }

    // Essayer de renommer d'abord
    match fs::rename(source_path, &final_destination) {
        Ok(_) => Ok(()),
        Err(_) => {
            // En cas d'échec, on fait une copie suivie d'une suppression
            fs::copy(source_path, &final_destination)?;
            fs::remove_file(source_path)?;
            Ok(())
        }
    }
}