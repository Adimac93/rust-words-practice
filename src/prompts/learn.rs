use crate::engine::Pool;
use crate::parser::parse_all;
use inquire::{InquireError, Select};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::SystemTime;

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

pub fn learn_prompt() -> anyhow::Result<()> {
    let parsed = parse_all()?;
    loop {
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
    }
    Ok(())
}
