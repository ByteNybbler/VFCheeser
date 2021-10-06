// Can only be used in functions or closures that return Result<(), Error>.
// This is because of the usage of ? inside the macro.
/*
#[macro_export]
macro_rules! text_args {
    ($command:ident, $($var:ident: $t:ty),*) => {
        let mut usage = format!("usage: /{}", $command.name());
        $(
            if (&&Wrap(<$t>::default())).is_option() {
                usage += &format!(" [{}]", stringify!($var));
            } else {
                usage += &format!(" {}", stringify!($var));
            }
        )*
        println!("{}", usage);

        let count_required = count_types_not_option!($($t),*);
        let count_maximum = count_types!($($t),*);
        let arguments = $command.arguments_required(count_required, count_maximum)?;
        let mut _index = 0;
        $(
            let $var = parse_argument!(&arguments, _index, $t)?;
            _index += 1;
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_text_command() {
        // TODO: If text_args is going to be used again, remove all the test lines duplicated from text_command.rs.

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

        (|| {
            text_args!(command, speed: u32);
            assert_eq!(speed, 3);
            Result::<(), Error>::Ok(())
        })().unwrap();

        let command = TextCommand::from_str("/me runs very fast.", '/').unwrap();
        assert_eq!(command.name(), "me");
        let arguments = command.arguments_required(0, 1).unwrap();
        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0], "runs very fast.");

        (|| {
            text_args!(command, message: &str);
            assert_eq!(message, "runs very fast.");
            text_args!(command, message: Option<&str>);
            assert_eq!(message, Some("runs very fast."));
            Result::<(), Error>::Ok(())
        })().unwrap();

        let command = TextCommand::from_str("/me", '/').unwrap();
        assert_eq!(command.name(), "me");
        assert_eq!(command.arguments_required(0, 1).unwrap().len(), 0);

        (|| {
            text_args!(command, message: Option<&str>);
            assert_eq!(message, None);
            Result::<(), Error>::Ok(())
        })().unwrap();

        let command = TextCommand::from_str("/velocity 3 5", '/').unwrap();
        assert_eq!(command.name(), "velocity");
        let arguments = command.arguments_required(2, 2).unwrap();
        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0], "3");
        assert_eq!(arguments[1], "5");

        (|| {
            text_args!(command, velocity_x: i32, velocity_y: i32);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);

            let command = TextCommand::from_str("/velocity 3 5 very cool", '/').unwrap();
            text_args!(command, velocity_x: i32, velocity_y: i32, blurb: Option<&str>);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);
            assert_eq!(blurb, Some("very cool"));

            let command = TextCommand::from_str("/velocity 3 5", '/').unwrap();
            text_args!(command, velocity_x: i32, velocity_y: i32, blurb: Option<&str>);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);
            assert_eq!(blurb, None);
            text_args!(command, velocity_x: i32, velocity_y: i32, velocity_z: Option<i32>, blurb: Option<&str>);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);
            assert_eq!(velocity_z, None);
            assert_eq!(blurb, None);

            let command = TextCommand::from_str("/velocity 3 5 ", '/').unwrap();
            text_args!(command, velocity_x: i32, velocity_y: i32, blurb: Option<&str>);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);
            assert_eq!(blurb, Some(""));

            let command = TextCommand::from_str("/velocity 3 5 7 ohhhh my god", '/').unwrap();
            text_args!(command, velocity_x: i32, velocity_y: i32, velocity_z: Option<i32>, blurb: Option<&str>);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);
            assert_eq!(velocity_z, Some(7));
            assert_eq!(blurb, Some("ohhhh my god"));

            let command = TextCommand::from_str("/velocity 3 5 7", '/').unwrap();
            text_args!(command, velocity_x: i32, velocity_y: i32, velocity_z: Option<i32>, blurb: Option<&str>);
            assert_eq!(velocity_x, 3);
            assert_eq!(velocity_y, 5);
            assert_eq!(velocity_z, Some(7));
            assert_eq!(blurb, None);

            Result::<(), Error>::Ok(())
        })().unwrap();
    }
}
*/