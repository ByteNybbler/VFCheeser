/*
pub struct ArgumentParser<'a> {
    position: usize,
    arguments: Vec<&'a str>
}

impl<'a> ArgumentParser<'a> {
    fn new(arguments: Vec<&str>) -> ArgumentParser {
        ArgumentParser {
            position: 0,
            arguments
        }
    }

    pub fn next<T>(&mut self) -> Option<Result<T, Error>>
    where
        T: FromStr
    {
        let result = self.arguments.get(self.position).map(|v| v.parse().map_err(|_| Error::ParseArgument{position: self.position}));
        self.position += 1;
        result
    }
}
*/