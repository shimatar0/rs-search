use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::index::{Index, PostingList};

pub struct IndexWriter {
    index_dir: String,
}

impl IndexWriter {
    pub fn new(path: String) -> Self {
        return IndexWriter { index_dir: path };
    }

    pub fn flush(self, index: Index) {
        for (term, posting_list) in index.dictionary.iter() {
            IndexWriter::flush_posting_list_writer(
                self.index_dir.clone(),
                term.to_string(),
                posting_list,
            );
        }
        IndexWriter::doc_count_writer(self.index_dir, index.total_docs_size);
    }

    pub fn flush_posting_list_writer(dir: String, term: String, list: &PostingList) {
        let data = serde_json::to_string(list).unwrap();
        let binding = PathBuf::from(dir).join(term);
        let file_name = binding.to_str().unwrap();
        match File::create(file_name) {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                println!("{}", data);
                writer.write(data.as_bytes()).unwrap();
                return;
            }
            Err(error) => {
                println!("{}", error);
                return;
            }
        }
    }

    pub fn doc_count_writer(dir: String, count: i32) {
        let binding = PathBuf::from(dir).join("_0.dc");
        let file_name = binding.to_str().unwrap();
        match File::create(file_name) {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                writer.write(count.to_string().as_bytes()).unwrap();
                return;
            }
            Err(error) => {
                println!("{}", error);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{index::DocumentID, indexer::Indexer, tokenizer::Tokenizer};

    use super::IndexWriter;

    #[test]
    fn flush() {
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

        let index_write = IndexWriter::new("./".to_string());
        index_write.flush(idxer.index);
    }
}
