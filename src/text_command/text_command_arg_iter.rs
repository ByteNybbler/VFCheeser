pub struct TextCommandArgIter<'a> {
    arguments_text: &'a str,
    arguments_chars: std::str::Chars<'a>,
    current_pos: usize,
    argument_count: usize,
    current_argument: usize
}

impl<'a> TextCommandArgIter<'a> {
    pub fn new(arguments_text: &str, argument_count: usize) -> TextCommandArgIter {
        TextCommandArgIter {
            arguments_text,
            arguments_chars: arguments_text.chars(),
            current_pos: 0,
            argument_count,
            current_argument: 0
        }
    }
}

impl<'a> Iterator for TextCommandArgIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos == self.arguments_text.len() {
            None
        } else {
            self.current_argument += 1;
            if self.current_argument <= self.argument_count {
                let mut starting_pos = self.current_pos;
                let mut quote_delimited = false;
                let mut skipping_whitespace_at_start = true;
                loop {
                    let c = self.arguments_chars.next();
                    if let Some(c) = c {
                        self.current_pos += 1;
                        if skipping_whitespace_at_start {
                            if c.is_whitespace() {
                                starting_pos += 1;
                            } else {
                                skipping_whitespace_at_start = false;
                            }
                        }
                        if !skipping_whitespace_at_start {
                            if self.current_pos-1 == starting_pos && c == '"' {
                                quote_delimited = true;
                                starting_pos += 1;
                            } else {
                                if
                                    (quote_delimited && c == '"') ||
                                    (!quote_delimited && self.current_argument < self.argument_count && c.is_whitespace())
                                {
                                    return Some(&self.arguments_text[starting_pos..self.current_pos-1])
                                }
                            }
                        }
                    } else {
                        return Some(&self.arguments_text[starting_pos..])
                    }
                }
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_command_arg_iter() {
        let mut iter = TextCommandArgIter::new("rust is cool", 3);
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("rust is cool", 4);
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("rust is cool yeah", 3);
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool yeah"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("rust is cool   ", 3);
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool   "));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new(" is cool", 3);
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("     rust      is      cool", 3);
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("\"rust is\" cool yeah", 3);
        assert_eq!(iter.next(), Some("rust is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), Some("yeah"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("\"rust is\" cool really \"yeah dude\" wow haha", 5);
        assert_eq!(iter.next(), Some("rust is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), Some("really"));
        assert_eq!(iter.next(), Some("yeah dude"));
        assert_eq!(iter.next(), Some("wow haha"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("   \" rust \" is cool", 3);
        assert_eq!(iter.next(), Some(" rust "));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), None);
        let mut iter = TextCommandArgIter::new("\"rust\"\"is\"\"cool\"", 3);
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), Some("is"));
        assert_eq!(iter.next(), Some("cool"));
        assert_eq!(iter.next(), None);
    }
}