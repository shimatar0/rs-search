use std::{env, fmt::write};

use crate::{
    document_store::DocumentStore, index_writer::IndexWriter, indexer::Indexer,
    tokenizer::Tokenizer,
};

pub struct Engine {
    tokenizer: Tokenizer,
    indexer: Indexer,
    document_store: DocumentStore,
    index_dir: String,
}

impl Engine {
    pub async fn new() -> Self {
        let tokenizer = Tokenizer::new();
        let indexer = Indexer::new(tokenizer);
        let document_store = DocumentStore::new();
        // create index data in "$pwd/_index_data/"
        let current = env::current_dir().unwrap();
        let path = current.join("_index_data");
        return Self {
            tokenizer,
            indexer,
            document_store: document_store.await,
            index_dir: path.into_os_string().into_string().unwrap(),
        };
    }

    pub async fn add_document(&mut self, title: String, collection: &[u8]) {
        let id = self.document_store.save(title).await;
        self.indexer.update(id, collection);
    }

    pub async fn flush(self) {
        let writer = IndexWriter::new(self.index_dir);
        writer.flush(self.indexer.index)
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;

    #[tokio::test]
    async fn engins_test() {
        let mut engine = Engine::new().await;

        struct TestDoc {
            title: String,
            body: String,
        }
        let docs = vec![
            TestDoc {
                title: "test1".to_string(),
                body: "Do you quarrel, sir?".to_string(),
            },
            TestDoc {
                title: "test2".to_string(),
                body: "No better.".to_string(),
            },
            TestDoc {
                title: "test3".to_string(),
                body: "Quarrel sir! no, sir!".to_string(),
            },
        ];

        for doc in docs {
            engine.add_document(doc.title, &doc.body.as_bytes()).await;
        }
        engine.flush().await;
    }
}
