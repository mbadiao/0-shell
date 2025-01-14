pub fn echo(args: Vec<String>) {
    let mut skip_newline = false;
    let mut start_index = 0;

    // Check for -n flag
    if !args.is_empty() && args[0] == "-n" {
        skip_newline = true;
        start_index = 1;
    }

    // Join arguments and remove quotes
    let output: String = args[start_index..]
        .iter()
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>()
        .join(" ");

    // Print with or without newline
    if skip_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
}
