use std::str::from_utf8;

#[derive(Debug, Clone, Copy)]
pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer {};
    }

    fn replace(&self, c: char) -> Option<char> {
        if !c.is_alphanumeric() {
            return None;
        }
        Some(c.to_ascii_lowercase())
    }

    pub fn split_func(&self, data: &[u8]) -> Vec<String> {
        let line = from_utf8(data).unwrap_or("").trim();
        line.split_whitespace()
            .map(|word| {
                word.chars()
                    .filter_map(|ch| self.replace(ch))
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn replace() {
        let tok = Tokenizer::new();
        let acctual = tok.replace('s');
        assert_eq!(acctual.unwrap(), 's');
        let acctual = tok.replace('A');
        assert_eq!(acctual.unwrap(), 'a');
        let acctual = tok.replace('@');
        assert_eq!(acctual, None);
    }

    #[test]
    fn split_func() {
        let tok = Tokenizer::new();
        let test = "it work test func!".as_bytes();
        let acctual = tok.split_func(test);
        assert_eq!(acctual, vec!["it", "work", "test", "func"]);
    }
}
