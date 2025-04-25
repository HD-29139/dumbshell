use std::env;
use colored::*;

fn _prompt_ () -> std::io::Result<String>{
let dir = env::current_dir()?;
// dir is a PathBuf,
// Retrieves the current working directory.
// the "?" propagates any error that may occur

let path = dir.file_name().and_then(|name| name.to_str()).unwrap_or("/");
let path_colored = format!("{}", path.magenta()); 
// file_name() returns the last component in the path, if it exists
let hour = chrono::Local::now().format("%H:%M").blue().to_string();

print!("{} {} mdshell> ",hour, path_colored);
stdout().flush().unwrap();
}