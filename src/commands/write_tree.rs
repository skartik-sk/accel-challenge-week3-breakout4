use crate::commands::hash_object::{
    build_object, compress_data, compute_hash, hex_to_sha_bytes, store_object,
};
use crate::error::ItError;
use crate::index::{IndexEntry, read_index};
use std::collections::BTreeMap;
use std::path::Path;

pub fn write_tree() -> Result<String, ItError> {
    let repo_path = Path::new(".it");
    let entries = read_index(repo_path)?;

    if entries.is_empty() {
        return Err(ItError::NothingToCommit);
    }

    build_tree(&entries, "")
}

//      prefix="" and path="src/main.rs" -> component="src", rest="main.rs"
//      prefix="" and path="README.md"   -> component="README.md", rest=""
fn build_tree(entries: &[IndexEntry], prefix: &str) -> Result<String, ItError> {
    let mut dirs: BTreeMap<String, Vec<&IndexEntry>> = BTreeMap::new();

    for entry in entries {
        let rel = if prefix.is_empty() {
            entry.path.as_str()
        } else {
            match entry.path.strip_prefix(&format!("{}/", prefix)) {
                Some(r) => r,
                None => continue,
            }
        };
        let component = rel.split('/').next().unwrap_or(rel).to_string();
        dirs.entry(component).or_default().push(entry);
    }

    let mut tree_content: Vec<u8> = Vec::new();

    for (name, group) in dirs {
        let is_file = group.len() == 1 && {
            let rel = if prefix.is_empty() {
                &group[0].path
            } else {
                group[0]
                    .path
                    .strip_prefix(&format!("{}/", prefix))
                    .unwrap_or(&group[0].path)
            };
            !rel.contains('/')
        };
        if is_file {
            let entry = group[0];
            let line = format!("100644 {}\0", name);

            tree_content.extend_from_slice(line.as_bytes());
            tree_content.extend_from_slice(&entry.sha);
        } else {
            let sub_prefix = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };
            let sub_hash = build_tree(entries, &sub_prefix)?;
            let line = format!("040000 {}\0", name);

            tree_content.extend_from_slice(line.as_bytes());
            tree_content.extend_from_slice(&hex_to_sha_bytes(&sub_hash));
        }
    }

    let tree_object = build_object(&tree_content, "tree");
    let hash = compute_hash(&tree_object);
    let compressed = compress_data(&tree_object)?;
    store_object(&hash, &compressed)?;

    println!("tree written: {}", hash);
    Ok(hash)
}
