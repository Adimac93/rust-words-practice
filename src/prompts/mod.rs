mod edit;
mod learn;

use crate::gui::start_gui_mode;
use crate::prompts::edit::edit_prompt;
use crate::prompts::learn::learn_prompt;
use inquire::{Confirm, InquireError, Select};

pub fn main_menu() -> anyhow::Result<()> {
    let options = vec!["Learn", "Add", "Settings", "GUI"];
    let main_select = Select::new("Main menu", options);
    loop {
        let menu = match main_select.clone().prompt() {
            Ok(ans) => ans,
            Err(error) => match error {
                InquireError::OperationCanceled => {
                    let exit = cursed_escape();
                    if exit {
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
        let menu = "GUI";

        return match menu {
            "Learn" => learn_prompt(),
            "Add" => edit_prompt(),
            "GUI" => start_gui_mode(),
            "Settings" => unreachable!(),
            _ => unreachable!(),
        };
    }
    Ok(())
}

fn cursed_escape() -> bool {
    let mut prompt = format!("Do you want to ESCape this program");
    loop {
        match Confirm::new(&prompt).prompt() {
            Ok(confirm) => return confirm,
            Err(error) => match error {
                InquireError::OperationCanceled => prompt.push_str(" cancel operation"),
                InquireError::OperationInterrupted => prompt.push_str(" interrupt operation"),
                _other => unreachable!(),
            },
        };
    }
}
