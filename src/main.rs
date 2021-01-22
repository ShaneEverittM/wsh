use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        if let Ok(0) = stdin().read_line(&mut input) {
            // EOF
            return;
        }

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // handle built-ins differently
        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/home/shane", |&x| x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let spawn_result = Command::new(command).args(args).spawn();

                match spawn_result {
                    Ok(mut child) => {
                        child.wait().expect("child wasn't running");
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
}
