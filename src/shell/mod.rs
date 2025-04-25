pub mod myprompt;

use colored::*;
use myprompt::gt_prompt;
use std::io::{Write, stdout};
use std::process::Command;

pub fn run_shell() -> std::io::Result<()> {
    loop {
        let prompt = gt_prompt()?;
        print!("{}", prompt);
        stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = match Command::new(command).args(args).spawn() {
            Ok(child) => child,
            Err(e) => {
                println!("aqui: {}", e.to_string().red());
                continue;
            }
        };
        child.wait()?;
    }

}
