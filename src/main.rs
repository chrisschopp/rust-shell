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
        let inputs: Vec<&str> = raw_input.split(' ').collect();
        let command = inputs[0];
        if command.eq("echo") {
            if inputs.len() > 1 {
                let arguments = &inputs[1].to_string();
                println!("{arguments}");
            } else {
            }
        } else if command.eq("exit") {
            break;
        } else {
            eprintln!("{}: command not found", command);
        }
        io::stdout().flush().unwrap();
    }
}
