use std::{env, path};
use std::io::{self, stdout, Write};
use std::process::Command;
use colored::Colorize;



fn main() -> std::io::Result<()>{
    loop {
       
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