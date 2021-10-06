#[macro_export]
macro_rules! text_commands {
    ($(
        $(#[alias($($alias:ident),*)])*
        $v:vis fn $func:ident(&mut $self:ident $(, $i:ident: $t:ty)*) $b:block
    )*) => {
        text_commands_generate_run_function!($($func [$($($alias)*)*])*);
        text_commands_generate_help_function!($($func [$($($alias)*)*])*);
        $(
            $v fn $func(&mut $self $(, $i: $t)*) $b
            text_commands_generate_per_command_functions!($func [$($($alias)*)*], $($i: $t),*);
        )*
    }
}

#[macro_export]
macro_rules! text_commands_generate_run_function {
    ($($func:ident [$($alias:ident)*])*) => {
        pub fn run_command(&mut self, command: &TextCommand) -> Result<(), Error> {
            match command.name() {
                $(
                    stringify!($func) => paste::paste!{self.[<$func _text_command>](command)},
                    $(
                        stringify!($alias) => paste::paste!{self.[<$func _text_command>](command)},
                    )*
                )*
                "help" => {
                    Self::help(command.single_str().ok());
                    Ok(())
                },
                "" => Err(Error::NoCommandNameProvided),
                other => Err(Error::CommandNotFound{name: other.to_owned()})
            }
        }   
    }
}

#[macro_export]
macro_rules! text_commands_generate_help_function {
    ($($func:ident [$($alias:ident)*])*) => {
        pub fn help(command_name: Option<&str>) {
            match command_name {
                Some(command_name) => match command_name {
                    $(
                        stringify!($func) => println!("usage: {}", paste::paste!{Self::[<$func _text_command_usage>]()}),
                        $(
                            stringify!($alias) => println!("usage: {}", paste::paste!{Self::[<$func _text_command_usage>]()}),
                        )*
                    )*
                    other => println!("{}", Error::CommandNotFound{name: other.to_owned()})
                },
                None => {
                    println!("/help [command_name]");
                    $(
                        println!("{}", paste::paste!{Self::[<$func _text_command_usage>]()});
                    )*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! text_commands_generate_per_command_functions {
    ($func:ident [$($alias:ident)*], $($i_text:ident: $t_text:ty),*) => {
        paste::paste!{
            fn [<$func _text_command>](&mut self, command: &$crate::text_command::TextCommand)
                -> Result<(), $crate::text_command::Error>
            {
                let count_required = $crate::count_types_not_option!($($t_text),*);
                let count_maximum = $crate::count_types!($($t_text),*);
                let _arguments = command.arguments_required(count_required, count_maximum)?;

                let mut _index = 0;
                self.$func($(
                        {
                            let arg = $crate::parse_argument!(&_arguments, _index, $i_text, $t_text)?;
                            _index += 1;
                            arg
                        }
                    ),*
                );

                Ok(())
            }
        }

        // TODO: Can this be calculated at compile time to return a &'static str?
        paste::paste!{
            fn [<$func _text_command_usage>]() -> String {
                let mut usage = "/".to_owned();
                if $crate::count_idents!($($alias),*) == 0 {
                    usage += &format!("{}", stringify!($func));
                } else {
                    usage += &format!("[{}", stringify!($func));
                    $(
                        usage += &format!(", {}", stringify!($alias));
                    )*
                    usage += "]";
                }
                $(
                    {
                        use $crate::macro_assist::*;
                        if (&&$crate::wrap_default!($t_text)).is_option() {
                            usage += &format!(" [{}]", stringify!($i_text));
                        } else {
                            usage += &format!(" {}", stringify!($i_text));
                        }
                    }
                )*
                usage
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::text_command::*;

    struct Runner;
    impl Runner {
        fn cool_func(&mut self) {

        }

        text_commands! {
            #[alias(first)]
            fn empty(&mut self) {
                self.cool_func();
            }
    
            #[alias(second, single)]
            fn one_arg(&mut self, _argument: u8) {

            }
    
            fn some_optionals(&mut self, _gun: u16, _opt1: Option<u8>, _opt2: Option<u8>, _blurb: Option<&str>) {
    
            }

            #[alias(tp)]
            fn teleport(&mut self, _player1: &str, _player2: &str) {

            }
        }
    }

    #[test]
    fn test_text_commands_macro_basic() {
        let mut runner = Runner;
        let command = TextCommand::from_str("/empty", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/noexist", '/').unwrap();
        assert!(runner.run_command(&command).is_err());
        let command = TextCommand::from_str("/one_arg", '/').unwrap();
        assert!(runner.run_command(&command).is_err());
        let command = TextCommand::from_str("/one_arg 16", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/some_optionals 8", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/some_optionals 8 9", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/some_optionals 8 9 10", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/some_optionals 8 9 10 hahaha hello hahaha", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/teleport \"My Cool Friend\" the other guy", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help one_arg", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help some_optionals", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help noexist", '/').unwrap();
        runner.run_command(&command).unwrap();
    }

    #[test]
    fn test_text_commands_macro_alias() {
        let mut runner = Runner;
        let command = TextCommand::from_str("/first", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/second 12", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/single 19", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help first", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help second", '/').unwrap();
        runner.run_command(&command).unwrap();
        let command = TextCommand::from_str("/help single", '/').unwrap();
        runner.run_command(&command).unwrap();
    }
}