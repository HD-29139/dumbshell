pub mod myprompt;
// pub mod commands;

// use commands::{color_cmd, handle_};
use myprompt::gt_prompt;
use std::env;
use std::io::{stdin, stdout, Error, Write};
use std::process::Command;
use colored::*;

pub fn run_shell() -> std::io::Result<()> {
    loop {
        let prompt = gt_prompt()?;
        print!("{}", prompt);
        stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = match parts.next() {
            Some(command) => command,
            None => continue,
        };
        let args = parts;

        let mut child = match Command::new(command).args(args).spawn() {
            Ok(child) => child,
            Err(e) => {
                println!("{}", e.to_string().red());
                continue;
            }
        };
        child.wait().unwrap();
    }
}
