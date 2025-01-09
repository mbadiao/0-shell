pub fn echo(args: &[&str]) {
    let mut skip_newline = false;
    let mut start_index = 0;
    
    if !args.is_empty() && args[0] == "-n" {
        skip_newline = true;
        start_index = 1;
    }

    let output: String = args[start_index..].join(" ");

    if skip_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
}
