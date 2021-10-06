use crate::macro_assist::*;
use std::str::FromStr;

mod error;
pub use error::*;
mod main_macro;
pub use main_macro::*;
mod parse_argument;
pub use parse_argument::*;
mod text_command_arg_iter;
pub use text_command_arg_iter::*;

pub struct TextCommand<'a>  {
    name: &'a str,
    arguments: Option<&'a str>
}

impl<'a> TextCommand<'a> {
    // Returns None if the provided string is not a text command with the given prefix.
    pub fn from_str(string: &str, prefix: char) -> Option<TextCommand> {
        if string.chars().nth(0) == Some(prefix) {
            let text_without_prefix = &string[1..];
            let mut iter = text_without_prefix.splitn(2, char::is_whitespace);
            let name = iter.next().unwrap();
            let arguments = iter.next();
            Some(TextCommand {
                name,
                arguments
            })
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn arguments(&self, max_argument_count: usize) -> Vec<&str> {
        //self.arguments.map_or(vec![], |arguments| arguments.splitn(max_argument_count, char::is_whitespace).collect())
        self.arguments.map_or(vec![], |arguments| TextCommandArgIter::new(arguments, max_argument_count).collect())
    }

    pub fn arguments_required(&self, required_argument_count: usize, max_argument_count: usize) -> Result<Vec<&str>, Error> {
        let argument_vector = self.arguments(max_argument_count);
        let provided_argument_count = argument_vector.len();
        if provided_argument_count >= required_argument_count {
            Ok(argument_vector)
        } else {
            Err(Error::NotEnoughArguments{required_argument_count, provided_argument_count})
        }
    }

    pub fn single_str(&self) -> Result<&str, Error> {
        let arguments = self.arguments_required(1, 1)?;
        Ok(arguments[0])
    }

    pub fn single_parse<T>(&self) -> Result<T, Error>
    where
        T: FromStr
    {
        self.single_str()?.parse().map_err(|_| Error::ParseArgument{name: "arg".to_owned()})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_command_struct() {
        let command = TextCommand::from_str("I am not a command.", '/');
        assert!(command.is_none());

        let command = TextCommand::from_str("/noclip", '/').unwrap();
        assert_eq!(command.name(), "noclip");
        assert_eq!(command.arguments(3).len(), 0);
        assert_eq!(command.arguments_required(0, 0).unwrap().len(), 0);

        let command = TextCommand::from_str("/speed 3", '/').unwrap();
        assert_eq!(command.name(), "speed");
        assert_eq!(command.arguments(1).len(), 1);
        assert_eq!(command.arguments_required(1, 1).unwrap().len(), 1);
        let speed: u32 = command.single_parse().unwrap();
        assert_eq!(speed, 3);

        let command = TextCommand::from_str("/me runs very fast.", '/').unwrap();
        assert_eq!(command.name(), "me");
        let arguments = command.arguments_required(0, 1).unwrap();
        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0], "runs very fast.");

        let command = TextCommand::from_str("/me", '/').unwrap();
        assert_eq!(command.name(), "me");
        assert_eq!(command.arguments_required(0, 1).unwrap().len(), 0);

        let command = TextCommand::from_str("/velocity 3 5", '/').unwrap();
        assert_eq!(command.name(), "velocity");
        let arguments = command.arguments_required(2, 2).unwrap();
        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0], "3");
        assert_eq!(arguments[1], "5");

        let command = TextCommand::from_str("/teleport \"My Cool Friend\" the other guy", '/').unwrap();
        assert_eq!(command.name(), "teleport");
        let arguments = command.arguments_required(2, 2).unwrap();
        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0], "My Cool Friend");
        assert_eq!(arguments[1], "the other guy");

        let command = TextCommand::from_str("/teleport \"My Cool Friend\" \"the other guy\" 3", '/').unwrap();
        assert_eq!(command.name(), "teleport");
        let arguments = command.arguments_required(2, 3).unwrap();
        assert_eq!(arguments.len(), 3);
        assert_eq!(arguments[0], "My Cool Friend");
        assert_eq!(arguments[1], "the other guy");
        assert_eq!(arguments[2], "3");
    }
}