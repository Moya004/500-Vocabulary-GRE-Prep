use crate::{
    db::repository::Repository,
    types::{NewWordEntry, Word},
};

#[derive(Debug)]
pub enum CommandReturnVariation {
    PostOperation(bool),
    GetOperation(Vec<Word>),
}

pub trait Command {
    fn execute(&self) -> Result<CommandReturnVariation, impl std::error::Error>;
}

#[derive(Debug)]
pub struct SaveNewWordCommand {
    pub new_word: NewWordEntry,
}

impl SaveNewWordCommand {
    pub fn new(new_word: NewWordEntry) -> Self {
        Self { new_word }
    }
}

impl Command for SaveNewWordCommand {
    fn execute(&self) -> Result<CommandReturnVariation, impl std::error::Error> {
        let repo = Repository::new().unwrap();
        let NewWordEntry {
            entry_input,
            class_input,
            pronunciation_input,
            definition_input,
            example_input,
        } = &self.new_word;
        let word = Word::new(
            entry_input.to_string(),
            class_input.to_string(),
            pronunciation_input.to_string(),
            definition_input.to_string(),
            example_input.to_string(),
        );

        if repo.exist_word(&word.entry)? {
            return Ok(CommandReturnVariation::PostOperation(false));
        }

        match repo.add_word(&word) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(CommandReturnVariation::PostOperation(true))
    }
}

pub struct NextRandomWordCommand;

impl Command for NextRandomWordCommand {
    fn execute(&self) -> Result<CommandReturnVariation, impl std::error::Error> {
        let repo = Repository::new().unwrap();

        let rand_word = repo.get_random_word();

        match rand_word {
            Ok(word) => Ok(CommandReturnVariation::GetOperation(vec![word])),
            Err(e) => Err(e),
        }
    }
}
