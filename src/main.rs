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
        let mut splitted_inputs: Vec<&str> = raw_input.split(' ').collect();
        let command = splitted_inputs.remove(0);
        match command {
            "exit" => break,
            "echo" => println!("{}", splitted_inputs.join(" ")),
            _ => println!("{command}: command not found"),
        }
        io::stdout().flush().unwrap();
    }
}
