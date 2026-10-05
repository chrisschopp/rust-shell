#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut raw_input = String::new();
        io::stdin().read_line(&mut raw_input).unwrap();
        raw_input = raw_input.trim().to_string();

        // Split to get command and any arguments passed
        let opts: Option<(&str, &str)> = raw_input.split_once(' ');
        match opts {
            Some((command, arguments)) => command_with_args(command, arguments),
            None => command_not_found(&raw_input),
        }
        if raw_input.eq("exit") {
            break;
        }
        io::stdout().flush().unwrap();
    }
}

fn command_with_args(command: &str, arguments: &str) {
    if command == "echo" {
        println!("{arguments}");
    } else {
        command_not_found(command);
    }
}

fn command_not_found(command: &str) {
    if command != "exit" {
        println!("{command}: command not found");
    }
}
