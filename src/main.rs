use inquire::validator::Validation;
use inquire::{Confirm, InquireError, Select, Text};
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use word_practice::engine::Pool;
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

struct PathPart(PathBuf);

impl PathPart {
    fn new(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }
}

impl Display for PathPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts: String = self
            .0
            .into_iter()
            .skip(2)
            .map(|str| str.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join("/");

        write!(f, "{parts}")
    }
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
                let options: Vec<PathPart> = parsed
                    .clone()
                    .into_keys()
                    .map(|part| PathPart::new(part))
                    .collect();

                let res = Select::new("Choose practice set", options).prompt();
                let ans = match res {
                    Ok(PathPart(ans)) => ans,
                    Err(error) => match error {
                        InquireError::OperationCanceled => break,
                        InquireError::OperationInterrupted => break,
                        other => {
                            println!("{other}");
                            break;
                        }
                    },
                };

                let key = PathBuf::from(ans);
                let set = parsed[&key].to_owned();
                let mut pool = Pool::new(set);
                let start = SystemTime::now();
                pool.cycle();
                let duration = start.elapsed().unwrap().as_secs();
                println!("Finished in {duration}s");
            },
            "Add" => {
                let file_name_prompt = Text::new("Enter a file name (without extension)");
                let file_name = file_name_prompt.prompt()?;
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
