use colored::*;

pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg.green());
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg.red());
}

pub fn info(msg: &str) {
    println!("{} {}", "i".blue().bold(), msg.blue());
}

pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
}

pub fn step(step_num: usize, total: usize, msg: &str) {
    println!("[{}/{}] {}", step_num, total, msg.cyan());
}

pub fn commit_hash(hash: &str) -> String {
    hash.bright_green().bold().to_string()
}

pub fn branch_name(name: &str) -> String {
    name.cyan().bold().to_string()
}

pub fn file_path(path: &str) -> String {
    path.white().to_string()
}
