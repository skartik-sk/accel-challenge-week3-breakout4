use colored::Colorize;

use crate::error::ItError;
use flate2::read::ZlibDecoder;
use std::io::Read;
use std::{fs, path::Path};

pub fn switch(branch_name: String) -> Result<(), ItError> {
    let repo_path = std::env::current_dir()?.join(".it");
    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(ItError::NotARepository);
    }
    let heads_path = repo_path.join("refs/heads");
    let target_branch_path = heads_path.join(&branch_name);

    if !target_branch_path.exists() {
        return Err(ItError::BranchNotFound(branch_name));
    }

    // read current HEAD to see if we're already on that branch
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
    let current_branch = head_content.trim_start_matches("ref: refs/heads/").trim();

    if current_branch == branch_name {
        println!("{} {}","already on".yellow(),branch_name.cyan());
        return Ok(());
    }

    let commit_target_branch = fs::read_to_string(&target_branch_path)?;
    let commit_hash = commit_target_branch.trim();

    let commit_content = read_object(&repo_path, commit_hash)?;

    let tree_hash = commit_content
        .lines()
        .find(|l| l.starts_with("tree "))
        .and_then(|l| l.strip_prefix("tree "))
        .ok_or(ItError::InvalidRef(commit_hash.to_string()))?
        .trim()
        .to_string();

    let cwd = std::env::current_dir()?;
    restore_tree(&repo_path, &tree_hash, &cwd)?;

    // point HEAD to the new branch
    fs::write(
        repo_path.join("HEAD"),
        format!("ref: refs/heads/{branch_name}\n"),
    )?;
    println!("{} {}","switched to branch".yellow(),branch_name.cyan());

    Ok(())
}

fn restore_tree(repo_path: &Path, tree_hash: &str, target_dir: &Path) -> Result<(), ItError> {
    let tree_content = read_object_raw(repo_path, tree_hash)?;
    let body_start = tree_content.iter().position(|&b| b == 0).unwrap_or(0) + 1;
    let body = &tree_content[body_start..];

    let mut i = 0;
    while i < body.len() {
        let null_pos = body[i..].iter().position(|&b| b == 0).unwrap() + i;
        let entry_header = std::str::from_utf8(&body[i..null_pos]).unwrap();
        let (mode, name) = entry_header.split_once(' ').unwrap();

        let hash_start = null_pos + 1;
        let hash_bytes = &body[hash_start..hash_start + 20];
        let hash_hex = hex::encode(hash_bytes);

        let entry_path = target_dir.join(name);

        if mode == "040000" {
            fs::create_dir_all(&entry_path)?;
            restore_tree(repo_path, &hash_hex, &entry_path)?;
        } else {
            let blob = read_object_raw(repo_path, &hash_hex)?;
            let body_start = blob.iter().position(|&b| b == 0).unwrap_or(0) + 1;
            fs::write(&entry_path, &blob[body_start..])?;
        }

        i = hash_start + 20;
    }

    Ok(())
}

fn read_object(repo_path: &Path, hash: &str) -> Result<String, ItError> {
    let bytes = read_object_raw(repo_path, hash)?;
    let body_start = bytes.iter().position(|&b| b == 0).unwrap_or(0) + 1;
    Ok(String::from_utf8_lossy(&bytes[body_start..]).to_string())
}

fn read_object_raw(repo_path: &Path, hash: &str) -> Result<Vec<u8>, ItError> {
    let path = repo_path.join("objects").join(&hash[0..2]).join(&hash[2..]);
    let compressed = fs::read(path)?;
    let mut decoder = ZlibDecoder::new(compressed.as_slice());
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}
