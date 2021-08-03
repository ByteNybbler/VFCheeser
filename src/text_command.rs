use std::collections::HashMap;

pub struct TextCommand;

impl TextCommand {
    pub fn execute(&self) {

    }
}

// Maps command names to commands.
pub struct TextCommandMapper {
    command_names: HashMap<String, TextCommand>
}

impl TextCommandMapper {
    pub fn new(command_names: HashMap<String, TextCommand>) -> TextCommandMapper {
        TextCommandMapper {
            command_names
        }
    }

    pub fn is_command_message(&self, message: &str) -> bool {
        message.chars().nth(0) == Some('/')
    }

    // Returns whether the message is a command message.
    pub fn try_run_command(&self, message: &str) -> bool {
        if self.is_command_message(message) {
            self.interpret_as_command(message);
            true
        } else {
            false
        }
    }

    fn get_message_without_command_prefix<'a>(&self, message: &'a str) -> &'a str {
        &message[1..]
    }

    fn split_message_into_arguments<'a>(&self, message: &'a str) -> Vec<&'a str> {
        let message_without_command_prefix = self.get_message_without_command_prefix(message);
        message_without_command_prefix.split_whitespace().collect()
    }

    fn interpret_as_command(&self, message: &str) -> Result<(), ()> {
        let arguments = self.split_message_into_arguments(message);
        self.parse_arguments(&arguments)
    }

    fn parse_arguments(&self, arguments: &[&str]) -> Result<(), ()> {
        let command_name = arguments[0];
        let maybe_command = self.command_names.get(command_name);
        if let Some(command) = maybe_command {
            command.execute();
            Ok(())
        } else {
            // No command under the given name.
            Err(())
        }
    }
}