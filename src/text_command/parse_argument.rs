use crate::parse::*;
use crate::text_command::*;

#[macro_export]
macro_rules! parse_argument {
    ($arguments:expr, $index:expr, $i:ident, $t:ty) => {
        {
            use $crate::text_command::*;
            (&&$crate::wrap_default!($t)).parse_argument($arguments, $index, stringify!($i))
        }
    };
}

pub trait ParseArgumentViaOption<'a, T> {
    fn parse_argument(&self, arguments: &'a Vec<&str>, index: usize, name: &str) -> Result<Option<T>, Error>;
}

impl<'a, T> ParseArgumentViaOption<'a, T> for &Wrap<Option<T>>
where
    T: MaybeFromStr<'a>
{
    fn parse_argument(&self, arguments: &'a Vec<&str>, index: usize, name: &str) -> Result<Option<T>, Error> {
        arguments.get(index).map(|v| v.maybe_parse().map_err(|_| Error::ParseArgument{name: name.to_owned()})).transpose()
    }
}

pub trait ParseArgumentViaAny<'a, T> {
    fn parse_argument(&self, arguments: &'a Vec<&str>, index: usize, name: &str) -> Result<T, Error>;
}

impl<'a, T> ParseArgumentViaAny<'a, T> for Wrap<T>
where
    T: MaybeFromStr<'a>
{
    fn parse_argument(&self, arguments: &'a Vec<&str>, index: usize, name: &str) -> Result<T, Error> {
        arguments[index].maybe_parse().map_err(|_| Error::ParseArgument{name: name.to_owned()})
    }
}