#[macro_export]
macro_rules! params_to_args {
    ($func:ident, $($i:ident: $t:ty),*) => {
        $func($($i),*)
    };
}

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