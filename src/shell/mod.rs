pub mod myprompt;

use myprompt::gt_prompt;
use std::env;
use std::io::{Write, stdout};
use std::path::Path;
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

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/home/ArchJefferson", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }

            command => {
                let mut child = Command::new(command).args(args).spawn()?;

                child.wait()?;
            }
        }
    }
}
