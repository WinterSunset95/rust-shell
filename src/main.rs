#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;

fn main() {

    // Infinite loop
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            break;
        } else {
            let output = command(input);
            println!("{}", output);
        }
    }
}

fn command(input: String) -> String {
    // Split the input into a vector of strings
    // The first element of the vector is the command
    // The rest of them are the arguments
    let input: Vec<&str> = input.split_whitespace().collect();

    // Below is a vector of currently supported commands
    // We will add more commands to this vector as we implement them
    let commands = vec!["echo", "exit", "type"];

    if input[0] == "echo" {

        // Command: echo
        // Join the arguments into a single string
        let output = input[1..].join(" ");
        return output;

    } else if input[0] == "type" {

        // Command: type
        // We might have more than one argument, so we join them into a single string
        let args = input[1..].join(" ");

        // Check if the command is supported
        if commands.contains(&args.as_str()) {
            return format!("{} is a shell builtin", args);
        } else {
            // For non-builtins
            // We gonna need to check the PATH environment variable
            let path = env::var("PATH").unwrap();
            let paths: Vec<&str> = path.split(":").collect();
            for p in paths {
                let file = format!("{}/{}", p, args);
                // Check if the file is executable
                if std::fs::metadata(file.clone()).is_ok() {
                    return format!("{} is {}", args, file);
                } else {
                    continue;
                }
            }

            return format!("{} not found", args);
        }

    } else {
        return format!("{}: command not found", input[0]);
    }
}
