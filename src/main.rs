use inquire::validator::Validation;
use inquire::{Confirm, InquireError, Select, Text};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
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
            .components()
            .into_iter()
            .filter_map(|x| match x {
                // Component::Prefix(_) => {}
                // Component::RootDir => {}
                // Component::CurDir => {}
                // Component::ParentDir => {}
                Component::Normal(name) => {
                    let name = name.to_owned().into_string().unwrap();
                    if name != SOURCE_FOLDER_NAME {
                        return Some("/".to_string() + &name);
                    }
                    None
                }
                _ => None,
            })
            .collect();

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
                match res {
                    Ok(ans) => {
                        let key = PathBuf::from(ans.0);
                        println!("{key:?}");
                        println!("{:?}", parsed.keys());
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
                let text = Text::new("Enter a file name (without extension)");
                let file_name = text.prompt()?;
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
