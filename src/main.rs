use std::io::{self, stdout, Write};
use std::process::Command;
fn main(){
    loop {
        print!("/dshell>");
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