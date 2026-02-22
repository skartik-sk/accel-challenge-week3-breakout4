use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;

pub struct IndexEntry {
    pub sha: [u8; 20],
    pub flags: u16,
    pub path: String,
}

pub fn read_index(repo_path: &Path) -> Result<Vec<IndexEntry>, std::io::Error> {
    let index_path = repo_path.join("index");
    if !index_path.exists() {
        return Ok(vec![]);
    }

    let data = fs::read(&index_path)?;
    if data.is_empty() {
        return Ok(vec![]);
    }

    if data.len() < 12 || &data[0..4] != b"DIRC" {
        println!("Invalid Object Format");
    }

    let entry_count = u32::from_be_bytes(data[8..12].try_into().unwrap()) as usize;
    let mut entries = Vec::with_capacity(entry_count);
    let mut pos = 12;

    for _ in 0..entry_count {
        if pos + 62 > data.len() {
            println!("Invalid Object Len");
        }

        let sha: [u8; 20] = data[pos + 40..pos + 60].try_into().unwrap();
        let flags = u16::from_be_bytes(data[pos + 60..pos + 62].try_into().unwrap());

        let path_start = pos + 62;
        let null_offset = data[path_start..].iter().position(|&b| b == 0).unwrap();

        let path = String::from_utf8(data[path_start..path_start + null_offset].to_vec()).unwrap();

        entries.push(IndexEntry { sha, flags, path });

        let entry_len = 62 + null_offset + 1; // fixed + path + null
        let padding = (8 - (entry_len % 8)) % 8;
        pos += entry_len + padding;
    }

    Ok(entries)
}

pub fn write_index(repo_path: &Path, entries: &[IndexEntry]) -> Result<(), std::io::Error> {
    let mut buf: Vec<u8> = Vec::new();

    buf.extend_from_slice(b"DIRC");
    buf.extend_from_slice(&2u32.to_be_bytes());
    buf.extend_from_slice(&(entries.len() as u32).to_be_bytes());

    for entry in entries {
        buf.extend_from_slice(&[0u8; 8]); // ctime
        buf.extend_from_slice(&[0u8; 8]); // mtime
        buf.extend_from_slice(&[0u8; 4]); // dev
        buf.extend_from_slice(&[0u8; 4]); // ino
        buf.extend_from_slice(&0o100644u32.to_be_bytes()); // mode
        buf.extend_from_slice(&[0u8; 4]); // uid
        buf.extend_from_slice(&[0u8; 4]); // gid
        buf.extend_from_slice(&[0u8; 4]); // size
        buf.extend_from_slice(&entry.sha); // sha1 (20 bytes)
        buf.extend_from_slice(&entry.flags.to_be_bytes());
        buf.extend_from_slice(entry.path.as_bytes());
        buf.push(0);

        let entry_len = 62 + entry.path.len() + 1;
        let padding = (8 - (entry_len % 8)) % 8;
        buf.extend_from_slice(&vec![0u8; padding]);
    }

    let checksum = Sha1::digest(&buf);
    buf.extend_from_slice(&checksum);

    fs::write(repo_path.join("index"), buf)?;
    Ok(())
}
