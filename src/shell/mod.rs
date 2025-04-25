pub mod _prompt_;
pub mod commands;

use commands::{color_cmd, handle_};
use prompt::get_prompt;
use std::env;
use std::io::{Write, stdin, stdout};
use std::process::Command;

fn run_shell() -> std::io::Result<()> {
    loop {
        let prompt = _prompt_();
        print!("{}", prompt);
        stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

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
