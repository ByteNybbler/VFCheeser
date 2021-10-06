use crate::parse::*;

pub trait MaybeFromOptionStr<'a, S: 'a>
where
    Self: Sized
{
    type Err;
    fn maybe_from_option_str(s: S) -> Result<Self, Self::Err>;
}

impl<'a, T> MaybeFromOptionStr<'a, &'a str> for T
    where T: MaybeFromStr<'a>
{
    type Err = <T as MaybeFromStr<'a>>::Err;
    fn maybe_from_option_str(s: &'a str) -> Result<T, Self::Err> {
        T::maybe_from_str(s)
    }
}

impl<'a, T> MaybeFromOptionStr<'a, Option<&'a str>> for Option<T>
    where T: MaybeFromStr<'a>
{
    type Err = <T as MaybeFromStr<'a>>::Err;
    fn maybe_from_option_str(s: Option<&'a str>) -> Result<Option<T>, Self::Err> {
        s.map(|v| T::maybe_from_str(v)).transpose()
    }
}

/*
impl<'a, T> MaybeFromOptionStr<'a, Option<&&'a str>> for Option<T>
    where T: MaybeFromStr<'a>
{
    type Err = <T as MaybeFromStr<'a>>::Err;
    fn maybe_from_option_str(s: Option<&&'a str>) -> Result<Option<T>, Self::Err> {
        s.map(|v| T::maybe_from_str(v)).transpose()
    }
}
*/

pub trait MaybeOptionParse {
    fn maybe_option_parse<'a, T>(&'a self) -> Result<T, T::Err>
    where
        Self: Sized,
        T: MaybeFromOptionStr<'a, Self>;
}

impl MaybeOptionParse for &str
{
    fn maybe_option_parse<'a, T>(&'a self) -> Result<T, T::Err>
    where
        T: MaybeFromOptionStr<'a, Self>
    {
        T::maybe_from_option_str(self)
    }
}

impl MaybeOptionParse for Option<&str>
{
    fn maybe_option_parse<'a, T>(&'a self) -> Result<T, T::Err>
    where
        T: MaybeFromOptionStr<'a, Self>
    {
        T::maybe_from_option_str(*self)
    }
}

/*
impl MaybeOptionParse for Option<&&str>
{
    fn maybe_option_parse<'a, T>(&'a self) -> Result<T, T::Err>
    where
        T: MaybeFromOptionStr<'a, Self>
    {
        T::maybe_from_option_str(*self)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_option_parse() {
        assert_eq!("4".maybe_option_parse::<u32>().unwrap(), 4);
        assert_eq!("-6".maybe_option_parse::<i32>().unwrap(), -6);
        assert_eq!("funny".maybe_option_parse::<&str>().unwrap(), "funny");
        assert_eq!(Some("4").maybe_option_parse::<Option<u32>>().unwrap(), Some(4));
        assert_eq!(Some("-6").maybe_option_parse::<Option<i32>>().unwrap(), Some(-6));
        assert_eq!(Some("funny").maybe_option_parse::<Option<&str>>().unwrap(), Some("funny"));
        assert_eq!(None.maybe_option_parse::<Option<u32>>().unwrap(), None);
        assert_eq!(None.maybe_option_parse::<Option<i32>>().unwrap(), None);
        assert_eq!(None.maybe_option_parse::<Option<&str>>().unwrap(), None);
    }
}