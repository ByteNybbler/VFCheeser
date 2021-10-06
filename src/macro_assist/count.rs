#[macro_export]
macro_rules! count_token_trees {
    () => (0usize);
    ( $t:tt $($ts:tt)* ) => (1usize + $crate::count_token_trees!($($ts)*));
}

#[macro_export]
macro_rules! count_idents {
    () => (0usize);
    ( $t:ident $(,$ts:ident)* ) => (1usize + $crate::count_idents!($($ts),*));
}

#[macro_export]
macro_rules! count_types {
    () => (0usize);
    ( $t:ty $(,$ts:ty)* ) => (1usize + $crate::count_types!($($ts),*));
}

#[macro_export]
macro_rules! count_types_not_option {
    () => (0usize);
    ( $t:ty $(,$ts:ty)* ) => {
        {
            use $crate::macro_assist::*;
            if (&&$crate::macro_assist::Wrap(<$t>::default())).is_option() {
                $crate::count_types_not_option!($($ts),*)
            } else {
                1usize + $crate::count_types_not_option!($($ts),*)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_macros() {
        assert_eq!(count_token_trees!(Option<u8>), 4);
        assert_eq!(count_idents!(ala, ala, ala, ala), 4);
        assert_eq!(count_types!(u8, i32, &str, Option<u8>, i8), 5);
        assert_eq!(count_types_not_option!(u8, i32, &str, Option<u8>, i8), 4);
    }
}