use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    NoCommandNameProvided,
    CommandNotFound{name: String},
    NotEnoughArguments{required_argument_count: usize, provided_argument_count: usize},
    ParseArgument{name: String}
}

impl Error {
    pub fn is_usage_error(&self) -> bool {
        match self {
            Self::NotEnoughArguments{..} => true,
            Self::ParseArgument{..} => true,
            _ => false
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoCommandNameProvided =>
                write!(f, "enter a command name after the prefix"),
            Error::CommandNotFound{name} =>
                write!(f, "no existing command named \"{}\"", name),
            Error::NotEnoughArguments{required_argument_count, provided_argument_count} =>
                write!(f, "{} argument(s) required; {} provided", required_argument_count, provided_argument_count),
            Error::ParseArgument{name} =>
                write!(f, "failed to parse argument \"{}\"", name)
        }
    }
}

impl std::error::Error for Error {}