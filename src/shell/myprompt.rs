use colored::*;
use std::env;

pub fn gt_prompt() -> std::io::Result<String> {
    let dir = env::current_dir()?;

    let path = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("/home/ArchJefferson");

    let path_colored = format!("{}", path.on_magenta().black());
    let prompt = "> ".yellow();
    let hour = chrono::Local::now()
        .format("%H:%M")
        .to_string()
        .blue()
        .bold();

    Ok(format!("[{}] {} {}", hour, path_colored, prompt))
}
