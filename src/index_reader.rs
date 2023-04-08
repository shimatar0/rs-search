use crate::index::PostingList;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

pub struct IndexReader {
    index_dir: String,
    postings_cache: HashMap<String, PostingList>,
    doc_count_cache: i32,
}

impl IndexReader {
    pub fn new(index_dir: String) -> Self {
        let cache = HashMap::new();
        return IndexReader {
            index_dir,
            postings_cache: cache,
            doc_count_cache: -1,
        };
    }

    pub fn postings_list(&mut self, terms: Vec<String>) -> Vec<PostingList> {
        let mut posting_lists: Vec<PostingList> = Vec::with_capacity(terms.len());
        for term in terms {
            let post =
                IndexReader::postings(&mut self.postings_cache, &term, self.index_dir.clone());
            posting_lists.push(post);
        }
        return posting_lists;
    }

    pub fn postings(
        posting_cache: &mut HashMap<String, PostingList>,
        term: &String,
        index_dir: String,
    ) -> PostingList {
        if posting_cache.contains_key(term) {
            let postiong_list = posting_cache.get(term).unwrap();
            return postiong_list.clone();
        } else {
            let file_path = PathBuf::from(index_dir).join(term);
            let mut file = File::open(file_path).expect("file none");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let posting_list: PostingList = serde_json::from_slice(&buffer).unwrap();

            // cache recreate
            posting_cache.insert((&term).to_string(), posting_list.clone());
            return posting_list;
        }
    }
}

#[cfg(test)]
mod test {
    use super::IndexReader;

    #[test]
    fn posting_list() {
        let mut idx_reader = IndexReader::new("./_index_data".to_string());
        idx_reader.postings_list(vec!["better".to_string(), "do".to_string()]);
    }
}
