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
    // Firstly, lets get the PATH environment variable
    // We will use this for non-builtins
    let path = env::var("PATH").unwrap();
    let paths: Vec<&str> = path.split(":").collect();

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
            // We gonna need to check the paths
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
        // Check if command is in PATH
        for p in paths {
            let file = format!("{}/{}", p, input[0]);
            if std::fs::metadata(file.clone()).is_ok() {
                // File is executable?
                // Execute the command
                let output = std::process::Command::new(file)
                    .args(&input[1..])
                    .output()
                    .expect("Failed to execute command");
                let return_string = String::from_utf8_lossy(&output.stdout).to_string();
                return format!("{}", return_string.trim());
            } else if std::fs::metadata(input[0]).is_ok() {
                // Command is not in PATH, but is executable
                let output = std::process::Command::new(input[0])
                    .args(&input[1..])
                    .output()
                    .expect("Failed to execute command");
                let return_string = String::from_utf8_lossy(&output.stdout).to_string();
                return format!("{}", return_string.trim());
            } else {
                // If the file is not executable, continue to the next path
                continue;
            }
        }

        // If the command is not found in PATH
        return format!("{}: command not found", input[0]);
    }
}
