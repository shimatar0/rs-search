use std::io::{BufReader, Cursor, Read};

use crate::{
    index::{DocumentID, Index},
    tokenizer::Tokenizer,
};

pub struct Indexer {
    index: Index,
    tokenizer: Tokenizer,
}

impl Indexer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        return Self {
            index: Index::new(),
            tokenizer,
        };
    }

    pub fn update(&self, doc_id: DocumentID, cursor: &[u8]) {
        let position: i32;
        let reader = BufReader::new(cursor);
        // tokens
        let tokens = self.tokenizer.split_func(cursor);
        for token in tokens {
            println!("{}", token);
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn update() {
        let collections = vec![
            "Do you quarrel, sir?",
            "Quarrel sir! no, sir!",
            "No better.",
            "Well, sir",
        ];
        // init indexer
        let toknizer = Tokenizer::new();
        let idxer = Indexer::new(toknizer);
        for (i, collection) in collections.iter().enumerate() {
            // let c = Cursor::new(collection.as_bytes());
            println!("{}:{}", i, collection);
            idxer.update(i as DocumentID, collection.as_bytes());
        }
    }
}
