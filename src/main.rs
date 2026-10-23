use faccess::PathExt;
use std::collections::HashSet;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
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
            _ => try_executing(command, &splitted_inputs),
        }
        io::stdout().flush().unwrap();
    }
}

fn get_type(inputs: &Vec<&str>) {
    let mut valid_commands = HashSet::new();
    valid_commands.insert("exit".to_string());
    valid_commands.insert("echo".to_string());
    valid_commands.insert("type".to_string());
    let command = inputs[0];
    // first check builtins
    if valid_commands.contains(command) {
        println!("{} is a shell builtin", command);
        if inputs.len() > 1 {
            // check all inputs passed to type
            get_type(&inputs[1..].to_vec());
        }
    } else {
        // next check executables
        let file_path = executable_path(command);
        match file_path {
            Some(f) => println!("{command} is {}", f.display()),
            None => not_found(command),
        }
    }
}

fn not_found(command: &str) {
    println!("{command}: not found")
}

fn executable_path(command: &str) -> Option<PathBuf> {
    // go through PATH
    let env_path = match env::var("PATH") {
        Ok(str) => str,
        Err(error) => panic!("No PATH variable found: {error:?}"),
    };
    let paths: Vec<PathBuf> = env::split_paths(&env_path)
        .map(|dir| dir.join(command))
        .collect();
    // check if file with command name exists and is executable
    let first_matching_file = paths.iter().find(|&file| file.exists() & file.executable());
    first_matching_file.cloned()
}

fn try_executing(command: &str, arguments: &Vec<&str>) {
    let file_path = executable_path(command);
    match file_path {
        Some(_) => execute(command, arguments),
        None => not_found(command),
    }
}

fn execute(command: &str, arguments: &Vec<&str>) {
    let output = Command::new(command)
        .args(arguments)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    print!("{stdout}");
}
