use colored::Colorize;

use crate::{ commands::log::log_branch, error::ItError};
use std::fs;
pub fn branch(name: Option<String>) -> Result<(), ItError> {
    let repo_path = std::env::current_dir()?.join(".it");
    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(ItError::NotARepository);
    }
    let heads_path = repo_path.join("refs/heads");

    match name {
        Some(branch_name) => {
            let new_branch_path = heads_path.join(branch_name.clone());
            if new_branch_path.exists() {
                return Err(ItError::BranchExists(branch_name));
            }

            let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
            let current_ref = head_content.trim_start_matches("ref: ").trim();
            let current_ref_path = repo_path.join(current_ref);

            if !current_ref_path.exists() {
                return Err(ItError::InvalidRef(current_ref.to_string()));
            }

            let current_hash = fs::read_to_string(current_ref_path)?;
            fs::write(new_branch_path, current_hash)?;
            
     

            log_branch(&branch_name)?;
            println!("{} {} ",  "branch".green(), format!("'{}' created", branch_name.cyan()).bold());

        }

        None => {
            let entries = fs::read_dir(heads_path)?;
            let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
            let current_branch = head_content.trim_start_matches("ref: refs/heads/").trim();

            for entry in entries {
                let entry = entry?;
                if current_branch.eq(entry.file_name().to_str().unwrap()) {
                    println!("{} {}","*".green(), entry.file_name().to_string_lossy().cyan().bold());
                } else {
                    println!("  {}", entry.file_name().to_string_lossy().white());
                }
            }
        }
    }

    Ok(())
}
