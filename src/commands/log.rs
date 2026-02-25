use std::{
    fs::{self, read_to_string},
    io::Write,
    time::{self, SystemTime, UNIX_EPOCH},
};

use colored::Colorize;

use crate::{commands::commit_tree::get_parent, error::ItError};

pub fn log() -> Result<(), ItError> {
    let curr_dir = std::env::current_dir()?;

    let repo_path = curr_dir.join(".it");
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;

    let current_branch_path = head_content.trim_start_matches("ref:").trim();

    let logs_path = repo_path.join("logs/");
    let current_branch_logs_path = logs_path.join(current_branch_path);

    // Handle missing log file gracefully
    let log_content = match fs::read_to_string(&current_branch_logs_path) {
        Ok(content) => content,
        Err(_) => {
            println!("{}", "ℹ No commits yet. Make your first commit with: it commit -m \"message\"".cyan().bold());
            return Ok(());
        }
    };

    for line in log_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
               
               if parts.len() >= 6 {
                   // Colorize hash (green)
                   let hash = parts[0].bright_green().bold();
                   
                   // Colorize parent hash (dimmed if zeros, yellow if has value)
                   let parent = if parts[1] == "0000000000000000000000000000000000000000" {
                       parts[1].dimmed()
                   } else {
                       parts[1].yellow()
                   };
                   
                   let dir = parts[2].cyan();
                   let time = parts[3].magenta();
                   let zone = parts[4].yellow();
                   
                   let message = line.split_whitespace().skip(5).collect::<Vec<_>>().join(" ");
                   let colored_msg = message
                       .replace("commit :", &"commit :".bright_blue().bold())
                       .replace("BRANCH FROM", &"BRANCH FROM".bright_cyan().bold());
                   
                   // Print with colors
                   println!("{} \n{} \n{} {} \n{} \n{} ",
                       hash, parent, time, zone, dir, colored_msg.white()
                   );
                   
                   // ADD NEWLINE HERE
                   println!();} else {
            println!("{}", line);
        }
    }


    Ok(())
}

// These are the function which need to be added when committing and branching
/// Format of out commit logs
/// last_commit current_commit_hash dir_name time zone commit : message
///
/// Save commit message :
///    - get the current_commit hash (we can fetch it)
///    - get the new_commit_hash    (provided)
///    - get the dir_name   ( we can get it)
///    - get time       -
///    - get zone       -
///    - get message    -

pub struct CommitArgs<'info> {
    new_commit_hash: &'info str,
    parent_commit_hash: Option<String>,
    dir_name: &'info str,
    time: u64,
    message: String,
}

pub fn form_commit_log(args: CommitArgs) -> String {
    // this is GMT
    let zone = "+5:30";
    let parent_display = args
        .parent_commit_hash
        .as_deref()
        .unwrap_or("0000000000000000000000000000000000000000");
    let log = format!(
        "{}",
        format_args!(
            "{} {} {} {} {} {}\n",
            args.new_commit_hash, parent_display, args.dir_name, args.time, zone, args.message
        )
    );

    log
}

pub fn branch_created_message(new_branch: &str, curr_branch: &str) -> String {
    format!(
        "{}",
        format_args!("{} {} -> {}","BRANCH FROM".bright_cyan().bold(), curr_branch.yellow(), new_branch.green().bold())
    )
}
pub fn commit_message(msg: &str) -> String {
    format!("{}", format_args!("{} {}","commit :".bright_blue().bold(), msg.white().bold()))
}

// log the commit
pub fn log_commit(
    new_commit_hash: &str,
    parent_commit_hash: Option<String>,
    message: &str,
) -> Result<(), ItError> {
    let curr_dir = std::env::current_dir()?;

    let repo_path = curr_dir.join(".it");
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;

    let current_branch_path = head_content.trim_start_matches("ref:").trim();

    //let current_branch_content = fs::read_to_string(repo_path.join(current_branch_path))?;
    //let current_commit_hash = &current_branch_content.trim();
    let dir_name = curr_dir.to_str().expect("Could not get the directory name");
    // this will give local machine time
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let message = commit_message(message);
    // this is for commit action
    let log_string = form_commit_log(CommitArgs {
        new_commit_hash,
        parent_commit_hash,
        dir_name,
        time,
        message,
    });

    let logs_path = repo_path.join("logs/");
    let current_branch_logs_path = logs_path.join(current_branch_path);

    let mut current_branch_log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(current_branch_logs_path)
        .expect("Cannot Open File");

    current_branch_log_file
        .write(log_string.as_bytes())
        .expect("Could not write to file");
    Ok(())
}

// log branch
pub fn log_branch(new_branch: &str) -> Result<(), ItError> {
    let curr_dir = std::env::current_dir()?;

    let repo_path = curr_dir.join(".it");
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;

    let current_branch_path = head_content.trim_start_matches("ref:").trim();

    let current_branch_content = fs::read_to_string(repo_path.join(current_branch_path))?;
    let current_commit_hash = &current_branch_content.trim();

    let dir_name = curr_dir.to_str().expect("Could not get the directory name");
    // this will give local machine time
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let current_branch_name = head_content.trim_start_matches("ref: refs/heads/").trim();

    // this is for branch action
    let log_string = form_commit_log(CommitArgs {
        new_commit_hash: current_commit_hash,
        parent_commit_hash: Some("0000000000000000000000000000000000000000".to_string()),
        dir_name,
        time,
        message: branch_created_message(new_branch, current_branch_name),
    });

    let logs_path = repo_path.join("logs/refs/heads");
    let new_branch_logs_path = logs_path.join(new_branch);

    let mut branch_log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(new_branch_logs_path)
        .expect("Cannot Open File");

    branch_log_file
        .write(log_string.as_bytes())
        .expect("Could not write to file");

    Ok(())
}
