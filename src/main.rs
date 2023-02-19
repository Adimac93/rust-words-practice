use std::collections::HashMap;
use inquire::{InquireError, Select};
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use anyhow::Context;

const SOURCE_DIR: &str = "./source";
const SPLIT_DELIMETER: &str = ";";

fn main() -> Result<(), anyhow::Error> {
    let path = Path::new(SOURCE_DIR);
    if path.exists() {
        let dir = fs::read_dir(SOURCE_DIR).context("Failed to read source directory")?;
        let sources = read_sources(dir);

        let parsed: HashMap<PathBuf, Vec<(String, String)>> = sources.iter().map(|source| (source.path(),parse(read_entry(source)))).collect();

        loop {
            let options: Vec<String> = parsed.keys().map(|key| key.display().to_string()).collect();
            let ans: Result<String, InquireError> = Select::new("Choose practice set", options).prompt();

            let key = PathBuf::from_str(&ans.unwrap()).unwrap();
            let set = parsed.get(&key).unwrap();
            println!("{set:#?}");
        }

    }
    fs::create_dir(SOURCE_DIR).context("Failed to create source directory")?;

    Ok(())
}

fn read_entry(entry: &DirEntry) -> String {
    let mut file = File::open(entry.path()).unwrap();
    let mut content = String::new();
    let res = file.read_to_string(&mut content).unwrap();
    content
}

fn parse(content: String) -> Vec<(String, String)> {
    let definitions: Vec<(String, String)> = content.lines().filter_map(|line| {
        if line.contains(SPLIT_DELIMETER) {
            if let Some((left, right)) = line.split_once(SPLIT_DELIMETER) {
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
