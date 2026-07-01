use crate::{
    logic::commands::{Command, NextRandomWordCommand},
    types::Word,
};

pub struct NextRandomWordCommandHandler;

impl NextRandomWordCommandHandler {
    pub fn execute_command(&self, cmd: NextRandomWordCommand) -> Option<Word> {
        let res = cmd.execute();

        let output = match res {
            Ok(variation) => match variation {
                super::commands::CommandReturnVariation::GetOperation(vec) => Some(vec[0].clone()),
                _ => None,
            },
            Err(_) => None,
        };

        output
    }
}
