#[macro_export]
macro_rules! params_to_args {
    ($func:ident, $($i:ident: $t:ty),*) => {
        $func($($i),*)
    };
}

/*
#[macro_export]
macro_rules! filter_params_not_maybe_from_option_string {
    (($callback:ident $($tokens:tt)*), => ($($passed_i:ident: $passed_t:ty)*)) => {
        $callback!($($tokens:tt)*, $($passed_i: $passed_t),*)
    };
    ($tt:tt, $i:ident: $t:ty, $($tail_i:ident: $tail_t:ty,)* => ($($passed_i:ident: $passed_t:ty)*)) => {
        // If this macro will ever be used, you need to make these if statements compile-time somehow.
        if (&&&wrap_default!($t)).is_maybe_from_option_str() {
            filter_params_not_maybe_from_option_string!(
                $tt, $($tail_i: $tail_t,)* => ($($passed_i: $passed_t)*)
            )
        } else {
            filter_params_not_maybe_from_option_string!(
                $tt, $($tail_i: $tail_t,)* => ($i: $t $($passed_i: $passed_t)*)
            )
        }
    };
    (@start $tt:tt, $($tail_i:ident: $tail_t:ty)*) => {
        $crate::filter_params_not_maybe_from_option_string!(
            $tt, $($tail_i: $tail_t,)* => ()
        );
    };
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_params_to_args() {
        fn testfunc(gun: u16, punk: Option<u8>, foil: &str) {
            assert_eq!(gun, 8);
            assert_eq!(punk, Some(12));
            assert_eq!(foil, "hello");
        }

        let gun = 8;
        let punk = Some(12);
        let foil = "hello";

        params_to_args!(testfunc, gun: u16, punk: Option<u8>, foil: &str);
    }
}