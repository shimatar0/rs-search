use crate::{document_store::DocumentStore, indexer::Indexer, tokenizer::Tokenizer};

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
        let path = "dir".to_string();
        return Self {
            tokenizer,
            indexer,
            document_store: document_store.await,
            index_dir: path,
        };
    }

    pub async fn add_document(&mut self, title: String, collection: &[u8]) {
        let id = self.document_store.save(title).await;
        self.indexer.update(id, collection);
    }
}
