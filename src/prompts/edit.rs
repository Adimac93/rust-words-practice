use crate::{SOURCE_FOLDER_NAME, SPLIT_DELIMITER};
use inquire::validator::Validation;
use inquire::{Confirm, InquireError, Text};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn edit_prompt() -> anyhow::Result<()> {
    let file_name_prompt = Text::new("Enter a file name (without extension)");
    let file_name = file_name_prompt.prompt()?;
    if file_name.is_empty() {
        println!("Seriously, empty? Use some creativity next time.");
        return Ok(());
    }
    let current_dir = std::env::current_dir()?;
    let mut full_path = current_dir
        .join(SOURCE_FOLDER_NAME)
        .join(Path::new(&file_name));
    full_path.set_extension("txt");
    println!("{full_path:?}");

    let mut is_appending = false;
    if full_path.exists() && full_path.is_file() {
        let confirm = Confirm::new("Would you like to append to the existing file?").prompt()?;
        is_appending = confirm;
    }

    let validator = |input: &str| match input.contains(SPLIT_DELIMITER) {
        false => {
            if input.is_empty() {
                return Ok(Validation::Invalid("You can not save empty value!".into()));
            }
            Ok(Validation::Valid)
        }
        true => Ok(Validation::Invalid(
            "You can not use split delimiter as part of your definition!".into(),
        )),
    };
    let ask_word = Text::new("Word").with_validator(validator);
    let ask_definition = Text::new("Definition").with_validator(validator);
    let mut buf: Vec<(String, String)> = Vec::new();
    println!("Click ESC to save definitions to file");
    loop {
        let word = match ask_word.clone().prompt() {
            Ok(word) => {
                if !word.is_empty() {
                    word
                } else {
                    println!("Enter a word!");
                    continue;
                }
            }
            Err(error) => match error {
                InquireError::OperationCanceled => break,
                other => {
                    println!("{other}");
                    return Ok(());
                }
            },
        };

        let definition = match ask_definition.clone().prompt() {
            Ok(definition) => {
                if !word.is_empty() {
                    definition
                } else {
                    println!("Enter a word!");
                    continue;
                }
            }
            Err(error) => match error {
                InquireError::OperationCanceled => break,
                other => {
                    println!("{other}");
                    return Ok(());
                }
            },
        };

        buf.push((word, definition));
        println!("Definition added");
    }

    if buf.is_empty() {
        println!("It seems you've just changed your mind, saving to file cancelled");
        return Ok(());
    }

    let mut file;
    if is_appending {
        file = fs::OpenOptions::new().append(true).open(&full_path)?;
    } else {
        file = File::create(&full_path)?;
    }

    let bytes = buf
        .iter()
        .flat_map(|(left, right)| format!("{left}{SPLIT_DELIMITER}{right}\n").into_bytes())
        .collect::<Vec<u8>>();
    file.write_all(bytes.as_slice())?;

    let count = buf.len();
    let def;
    if count == 1 {
        def = "definition"
    } else {
        def = "definitions"
    }
    println!(
        "Saved {count} {def} to file {:?}",
        &full_path.file_name().unwrap()
    );
    Ok(())
}
