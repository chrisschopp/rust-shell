use faccess::PathExt;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::{env, fs, path};

#[derive(Debug, PartialEq)]
enum Cmd {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl FromStr for Cmd {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Cmd::Exit),
            "echo" => Ok(Cmd::Echo),
            "type" => Ok(Cmd::Type),
            "pwd" => Ok(Cmd::Pwd),
            "cd" => Ok(Cmd::Cd),
            unknown_cmd => Err(format!("Unknown command {}", unknown_cmd)),
        }
    }
}

struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    fn change_directory(&mut self, arguments: &Vec<&str>) {
        let new_dir = match arguments.first() {
            Some(p) => p,
            None => "~",
        };
        let path = PathBuf::from_str(new_dir).unwrap(); // can unwrap since Infallible
        let path_is_absolute = path.exists() & path.is_absolute();
        if path_is_absolute {
            self.current_dir = path.clone();
        } else {
            self.try_as_relative_path(path);
        }
    }

    fn try_as_relative_path(&mut self, path: PathBuf) {
        let new_abs_path = path::absolute(self.current_dir.join(&path));
        if new_abs_path.is_ok() {
            let new_canon_path = fs::canonicalize(new_abs_path.unwrap());
            if new_canon_path.as_ref().is_ok_and(|p: &PathBuf| p.exists()) {
                self.current_dir = new_canon_path.unwrap();
            } else if path.to_str().is_some_and(|p| p.eq("~")) {
                self.current_dir = env::home_dir().expect("No home directory found.");
            } else {
                println!("cd: {}: No such file or directory", path.display());
            }
        } else {
            println!("cd: {}: No such file or directory", path.display());
        }
    }
}

fn main() {
    let mut shell = Shell {
        current_dir: std::env::current_dir()
            .expect("The parent of this shell should have a working directory"),
    };
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut raw_input = String::new();
        io::stdin().read_line(&mut raw_input).unwrap();
        raw_input = raw_input.trim().to_string();

        // Split to get command and any arguments passed
        let mut splitted_inputs: Vec<&str> = raw_input.split(' ').collect();
        let command = splitted_inputs.remove(0);
        match command.parse::<Cmd>() {
            Ok(Cmd::Exit) => break,
            Ok(Cmd::Echo) => println!("{}", splitted_inputs.join(" ")),
            Ok(Cmd::Type) => get_type(&splitted_inputs),
            Ok(Cmd::Pwd) => println!("{}", shell.current_dir.display()),
            Ok(Cmd::Cd) => shell.change_directory(&splitted_inputs),
            _ => try_executing(command, &splitted_inputs),
        }
        io::stdout().flush().unwrap();
    }
}

fn get_type(inputs: &Vec<&str>) {
    let command = inputs[0];
    // first check builtins
    if Cmd::from_str(command).is_ok() {
        println!("{} is a shell builtin", command);
        if inputs.len() > 1 {
            // check all inputs passed to type
            get_type(&inputs[1..].to_vec());
        }
    } else {
        // next check executables
        match path_to_executable(command) {
            Some(f) => println!("{command} is {}", f.display()),
            None => not_found(command),
        }
    }
}

fn not_found(command: &str) {
    println!("{command}: not found")
}

fn path_to_executable(command: &str) -> Option<PathBuf> {
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
    match path_to_executable(command) {
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
