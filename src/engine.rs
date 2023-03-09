use inquire::{InquireError, Text};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Pool {
    definitions: Vec<(String, String)>,
}

impl Pool {
    pub fn new(definitions: Vec<(String, String)>) -> Self {
        let mut rng = thread_rng();
        let mut def = definitions;
        def.shuffle(&mut rng);
        Self { definitions: def }
    }

    fn ask(&mut self, left: &str, right: &str) -> bool {
        let message = format!("{right}:");
        let text = Text::new(&message).with_placeholder("translation");

        match text.prompt() {
            Ok(guess) => {
                if guess.trim().to_lowercase() == left {
                    println!("Your are correct");
                    true
                } else {
                    println!("You are wrong, correct answer is {left}");
                    false
                }
            }
            Err(error) => match error {
                InquireError::OperationCanceled => {
                    println!("Skipping");
                    false
                }
                other => panic!("{}", other.to_string()),
            },
        }
    }

    pub fn cycle(&mut self) {
        println!("Translate");
        let mut round = 1;
        let mut total = self.definitions.len();
        while !self.definitions.is_empty() {
            println!("Round {round}");
            let mut missed: Vec<(String, String)> = Vec::new();
            while let Some((left, right)) = self.definitions.pop() {
                let is_correct = self.ask(&left, &right);
                if !is_correct {
                    missed.push((left, right));
                }
            }
            let mut rng = thread_rng();
            missed.shuffle(&mut rng);
            self.definitions = missed;
            let completed = total - self.definitions.len();
            println!("{completed}/{total}");
            total = self.definitions.len();
            round += 1;
        }
    }
}
