use std::collections::HashSet;
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
            "type" => get_type(&splitted_inputs),
            _ => println!("{command}: command not found"),
        }
        io::stdout().flush().unwrap();
    }
}

fn get_type(inputs: &Vec<&str>) {
    let mut valid_commands = HashSet::new();
    valid_commands.insert("exit".to_string());
    valid_commands.insert("echo".to_string());
    valid_commands.insert("type".to_string());
    if valid_commands.contains(inputs[0]) {
        println!("{} is a shell builtin", inputs[0])
    } else {
        println!("{}: not found", inputs[0])
    }
}
