use crate::{
    index::{DocumentID, Index, Posting, PostingList},
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

    pub fn update(&mut self, doc_id: DocumentID, collection: &[u8]) {
        let mut position: i32 = 0;
        let terms = self.tokenizer.split_func(collection);
        for term in terms {
            if !self.index.dictionary.contains_key(&term) {
                let postings = [Posting::new(doc_id, [position].to_vec())].to_vec();
                self.index
                    .dictionary
                    .insert(term, PostingList::new(postings));
            } else {
                let posting_list = self.index.dictionary.get_mut(&term).unwrap();
                posting_list.add(Posting::new(doc_id, [position].to_vec()));
            };
            position += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
        let mut idxer = Indexer::new(toknizer);
        for (i, collection) in collections.iter().enumerate() {
            idxer.update(i as DocumentID, collection.as_bytes());
        }

        let actual = idxer.index;

        let mut posting_list_map: HashMap<String, PostingList> = HashMap::new();
        posting_list_map.insert(
            "better".to_string(),
            PostingList::new(vec![Posting::new(2, vec![1])]),
        );
        posting_list_map.insert(
            "do".to_string(),
            PostingList::new(vec![Posting::new(0, vec![0])]),
        );
        posting_list_map.insert(
            "no".to_string(),
            PostingList::new(vec![Posting::new(1, vec![2]), Posting::new(2, vec![0])]),
        );
        posting_list_map.insert(
            "quarrel".to_string(),
            PostingList::new(vec![Posting::new(0, vec![2]), Posting::new(1, vec![0])]),
        );
        posting_list_map.insert(
            "sir".to_string(),
            PostingList::new(vec![
                Posting::new(0, vec![3]),
                Posting::new(1, vec![1, 3]),
                Posting::new(3, vec![1]),
            ]),
        );
        posting_list_map.insert(
            "well".to_string(),
            PostingList::new(vec![Posting::new(3, vec![0])]),
        );
        posting_list_map.insert(
            "you".to_string(),
            PostingList::new(vec![Posting::new(0, vec![1])]),
        );
        let expected = Index {
            dictionary: posting_list_map,
            total_docs_size: 0 as i32,
        };
        assert_eq!(expected, actual);
    }
}
