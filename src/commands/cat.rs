use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn cat(args: &[&str]) {
    if args.is_empty() {
        println!("cat: missing file operand");
        return;
    }

    for filename in args {
        if let Err(err) = print_file(filename) {
            println!("cat: {}: {}", filename, err);
        }
    }
}

fn print_file(filename: &str) -> io::Result<()> {
    // Ouvrir le fichier
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Lire et afficher chaque ligne
    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}