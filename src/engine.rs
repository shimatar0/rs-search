use crate::{
    document_store::DocumentStore,
    indexer::{self, Indexer},
    tokenizer::Tokenizer,
};

pub struct Engine {
    tokenizer: Tokenizer,
    indexer: Indexer,
    document_store: DocumentStore,
    index_dir: String,
}

impl Engine {
    pub fn new() {
        let tokenizer = Tokenizer::new();
        let indexer = Indexer::new(tokenizer);
    }
}
