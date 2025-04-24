use std::env;
use std::io::{self, stdout, Write};
use std::process::Command;
const MAGENTA_ITALIC: &str= "\x1b[4;3;35m";
const BLUE: &str= "\x1b[34";
const RESET: &str= "\x1b[0m";


fn main() -> std::io::Result<()>{
    loop {
        let dir = env::current_dir()?;
        // dir is a PathBuf,
        // Retrieves the current working directory.
        // the "?" propagates any error that may occur

        let path = dir.file_name().and_then(|name| name.to_str()).unwrap_or("/");
        // file_name() returns the last component in the path, if it exists

        print!("{}{}{} {}mdshell{}> ", MAGENTA_ITALIC, path, RESET, BLUE, RESET);
        stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = match parts.next() {
            Some(command) => command,
            None => continue
        };
        let args = parts;

        let mut child = match Command::new(command).args(args).spawn() {
            Ok(child) => child,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        child.wait().unwrap();
    }
}