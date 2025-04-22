use std::io::{self, stdout, Write};
use std::process::Command;
fn main(){
    loop {
        print!("dumbshell>");
        stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();


        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        if command == "quit" || command == "q" {
            break;
        }else {
            Command::new(command).args(args).status().unwrap();
        }
    }
}