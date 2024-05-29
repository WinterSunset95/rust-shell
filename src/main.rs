#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    // print!("$ ");
    // io::stdout().flush().unwrap();

    // Wait for user input
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

    // Command: echo
    if input[0] == "echo" {
        // Join the arguments into a single string
        let output = input[1..].join(" ");
        return output;
    } else {
        return format!("{}: command not found", input[0]);
    }
}
