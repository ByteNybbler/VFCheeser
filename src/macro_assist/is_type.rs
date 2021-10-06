use crate::macro_assist::*;
use crate::parse::*;

pub trait IsOption {
    fn is_option(&self) -> bool;
}

impl<T> IsOption for &Wrap<Option<T>> {
    fn is_option(&self) -> bool {
        true
    }
}

impl<T> IsOption for Wrap<T> {
    fn is_option(&self) -> bool {
        false
    }
}

pub trait IsStringSliceMaybeOption {
    fn is_string_slice_maybe_option(&self) -> bool;
}

impl IsStringSliceMaybeOption for &&Wrap<Option<&str>> {
    fn is_string_slice_maybe_option(&self) -> bool {
        true
    }
}

impl IsStringSliceMaybeOption for &Wrap<&str> {
    fn is_string_slice_maybe_option(&self) -> bool {
        true
    }
}

impl<T> IsStringSliceMaybeOption for Wrap<T> {
    fn is_string_slice_maybe_option(&self) -> bool {
        false
    }
}

pub trait IsMaybeFromOptionStr {
    fn is_maybe_from_option_str(&self) -> bool;
}

impl<'a, T> IsMaybeFromOptionStr for &&Wrap<T>
where
    T: MaybeFromOptionStr<'a, Option<&'a str>>
{
    fn is_maybe_from_option_str(&self) -> bool {
        true
    }
}

impl<'a, T> IsMaybeFromOptionStr for &Wrap<T>
where
    T: MaybeFromOptionStr<'a, &'a str>
{
    fn is_maybe_from_option_str(&self) -> bool {
        true
    }
}

impl<T> IsMaybeFromOptionStr for Wrap<T> {
    fn is_maybe_from_option_str(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_option() {
        assert!((&&Wrap(Some(8))).is_option());
        assert!(!(&&Wrap(8)).is_option());
    }

    #[test]
    fn test_is_string_slice_maybe_option() {
        assert!((&&&Wrap("asdf")).is_string_slice_maybe_option());
        assert!((&&&Wrap(Some("asdf"))).is_string_slice_maybe_option());
        assert!((&&&Wrap(None)).is_string_slice_maybe_option());
        assert!(!(&&&Wrap(8)).is_string_slice_maybe_option());
        assert!(!(&&&Wrap(Option::<u8>::None)).is_string_slice_maybe_option());
    }

    #[test]
    fn test_is_maybe_from_option_str() {
        assert!((&&&Wrap("asdf")).is_maybe_from_option_str());
        assert!((&&&Wrap(Some("asdf"))).is_maybe_from_option_str());
        assert!((&&&Wrap(8)).is_maybe_from_option_str());
        assert!((&&&Wrap(Some(8))).is_maybe_from_option_str());
        assert!(!(&&&Wrap(&8)).is_maybe_from_option_str());
    }
}