use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn cat(args: Vec<String>) {
    if args.is_empty() {
        println!("cat: missing file operand");
        return;
    }

    for filename in args {
        if let Err(err) = print_file(&filename) {
            println!("cat: {}: {}", filename, err);
        }
    }
}

fn print_file(filename: &str) -> io::Result<()> {
    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Read and print each line
    for line in reader.lines() {
        match line {
            Ok(content) => print!("{}", content),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}