use inquire::{Confirm, InquireError, Select, Text};
use std::fs;
use std::fs::{File};
use std::io::Write;
use std::path::{Path, PathBuf};
use word_practice::engine::{Pool};
use word_practice::parser::parse_all;
use word_practice::{SOURCE_FOLDER_NAME, SPLIT_DELIMITER};

fn cursed_escape(prompt: String) -> bool {
    let confirm = match Confirm::new(&format!("Do you want to ESCape {prompt}?")).prompt() {
        Ok(confirm) => confirm,
        Err(error) => match error {
            InquireError::OperationCanceled => cursed_escape(prompt + " cancel operation"),
            InquireError::OperationInterrupted => cursed_escape(prompt + " interrupt operation"),
            _other => unreachable!(),
        },
    };
    confirm
}

fn main() -> Result<(), anyhow::Error> {
    let parsed = parse_all()?;

    let options = vec!["Learn", "Add", "Settings"];
    let main_select = Select::new("Main menu", options);
    loop {
        let menu = match main_select.clone().prompt() {
            Ok(ans) => ans,
            Err(error) => match error {
                InquireError::OperationCanceled => {
                    let confirm = cursed_escape("this program".into());
                    if confirm {
                        break;
                    }
                    continue;
                }
                other => {
                    println!("{other}");
                    break;
                }
            },
        };
        match menu {
            "Learn" => loop {
                let options: Vec<String> =
                    parsed.keys().map(|key| key.display().to_string()).collect();
                let res = Select::new("Choose practice set", options).prompt();
                match res {
                    Ok(ans) => {
                        let key = PathBuf::from(ans);
                        let set = parsed.get(&key).unwrap().to_owned();
                        let mut pool = Pool::new(set);
                        pool.cycle();
                    }
                    Err(error) => match error {
                        InquireError::OperationCanceled => break,
                        InquireError::OperationInterrupted => return Ok(()),
                        other => {
                            println!("{other}");
                            break;
                        }
                    },
                }
            },
            "Add" => {
                let file_name = Text::new("Enter a file name (without extension)").prompt()?;
                if file_name.is_empty() {
                    println!("Seriously, empty? Use some creativity next time.");
                    continue;
                }
                let current_dir = std::env::current_dir()?;
                let mut full_path = current_dir
                    .join(SOURCE_FOLDER_NAME)
                    .join(Path::new(&file_name));
                full_path.set_extension("txt");
                println!("{full_path:?}");

                let mut is_appending = false;
                if full_path.exists() && full_path.is_file() {
                    let confirm =
                        Confirm::new("Would you like to append to the existing file?").prompt()?;
                    is_appending = confirm;
                }

                let ask_word = Text::new("Word");
                let ask_definition = Text::new("Definition");
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
                    continue;
                }

                let mut file;
                if is_appending {
                    file = fs::OpenOptions::new().append(true).open(&full_path)?;
                } else {
                    file = File::create(&full_path)?;
                }

                let bytes = buf
                    .iter()
                    .flat_map(|(left, right)| {
                        format!("{left}{SPLIT_DELIMITER}{right}\n").into_bytes()
                    })
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
            }
            "Settings" => {}
            _ => unreachable!(),
        }
    }

    println!("See ya later!");
    Ok(())
}
