use crate::shell;
use gethostname::gethostname;
use home::home_dir;
use shell::getgit::git_branch;
use colored::*;
use std::env;


pub fn gt_prompt() -> std::io::Result<String> {
    let user = whoami::username();
    let host = gethostname().to_string_lossy().to_string();
    let branch = git_branch();
    let ugly = String::from("/home/") + &user;
    let dir = env::current_dir()?;
    let path = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(&ugly);

    let path_colored = format!("{}", path.truecolor(114, 135, 253));
    let prompt = "> ".yellow();
    let hour = chrono::Local::now()
        .format("%H:%M")
        .to_string()
        .blue()
        .bold();

    let line = "\u{256D}";
    let end_line = "\u{2570}\u{25B8} ";
    
    if let Some(branch_name) = branch  {
        Ok(format!("{}[{}] {}{}{} {}{} \n{}\u{231C}{}\u{231f} {}", line.truecolor(220, 138, 120),hour, user.truecolor(210, 15, 57), "@".truecolor(114, 135, 253), host.truecolor(254, 100, 11), "\u{E0A0}".truecolor(244, 184, 228),branch_name.truecolor(136, 57, 239), end_line.truecolor(220, 138, 120),path_colored, prompt))
    }else {
        Ok(format!("{}[{}] {}{}{} \n{}\u{231C}{}\u{231f} {}", line.truecolor(220, 138, 120),hour, user.truecolor(210, 15, 57), "@".truecolor(114, 135, 253), host.truecolor(254, 100, 11), end_line.truecolor(220, 138, 120), path_colored, prompt))
    }
}
