use crate::macro_assist::*;
use std::str::FromStr;

pub trait MaybeFromStr<'a>
where
    Self: Sized + 'a
{
    type Err;
    fn maybe_from_str(s: &'a str) -> Result<Self, Self::Err>;
}

impl<'a> MaybeFromStr<'a> for &'a str {
    type Err = ();
    fn maybe_from_str(s: &'a str) -> Result<&'a str, Self::Err> {
        Ok(s)
    }
}

macro_rules! impl_maybe_from_str {
    ($t:ty) => {
        impl MaybeFromStr<'_> for $t {
            type Err = <$t as FromStr>::Err;
            fn maybe_from_str(s: &str) -> Result<$t, Self::Err> {
                s.parse()
            }
        }
    }
}

populate_impl!(impl_maybe_from_str, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String);

pub trait MaybeParse {
    fn maybe_parse<'a, T>(&'a self) -> Result<T, T::Err>
    where
        T: MaybeFromStr<'a>;
}

impl MaybeParse for &str
{
    fn maybe_parse<'a, T>(&'a self) -> Result<T, T::Err>
    where
        T: MaybeFromStr<'a>
    {
        T::maybe_from_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_parse() {
        assert_eq!("4".maybe_parse::<u32>().unwrap(), 4);
        assert_eq!("-6".maybe_parse::<i32>().unwrap(), -6);
        assert_eq!("funny".maybe_parse::<&str>().unwrap(), "funny");
    }
}