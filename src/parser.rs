use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};
use anyhow::Context;
use crate::{SPLIT_DELIMITER,SOURCE_FOLDER_NAME};


pub fn parse_all() -> anyhow::Result<HashMap<PathBuf, Vec<(String, String)>>> {

    let path = std::env::current_dir()?.join(SOURCE_FOLDER_NAME);
    if path.exists() {
        let dir = fs::read_dir(path).context("Failed to read source directory")?;
        let sources = read_sources(dir);

        let parsed: HashMap<PathBuf, Vec<(String, String)>> = sources.iter().map(|source| (source.path(),parse_file(read_entry(source)))).collect();
        return Ok(parsed);


    }
    fs::create_dir(path).context("Failed to create source directory")?;
    Ok(HashMap::new())
}

fn read_entry(entry: &DirEntry) -> String {
    let mut file = File::open(entry.path()).unwrap();
    let mut content = String::new();
    let res = file.read_to_string(&mut content).unwrap();
    content
}

fn parse_file(content: String) -> Vec<(String, String)> {
    let definitions: Vec<(String, String)> = content.lines().filter_map(|line| {
        if line.contains(SPLIT_DELIMITER) {
            if let Some((left, right)) = line.split_once(SPLIT_DELIMITER) {
                return Some((left.trim().to_owned(), right.trim().to_owned()));
            }
        }
        return None;
    }).collect();
    definitions
}

fn read_sources(dir: ReadDir) -> Vec<DirEntry>{
    let entries: Vec<DirEntry> = dir.filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
    let mut buf: Vec<DirEntry> = Vec::new();
    for entry in entries {
        let Ok(file_type) = entry.file_type() else {
            continue
        };
        if file_type.is_file() {
            buf.push(entry);
        } else if file_type.is_dir() {

            let Ok(dir) = fs::read_dir(entry.path()) else {
                continue
            };
            buf.extend(read_sources(dir));
        }
    }
    buf
}