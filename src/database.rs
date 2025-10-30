use crate::multimap::ConcurrentMultiMap;
use std::sync::Mutex;

// The archive struct contains two data structures: a ConcurrentMultiMap for storing the
// reverse index that maps words to the documents they appear in, and a Mutex<Vec<String>> for
// storing the documents themselves. Since the documents themselves aren't accessed as often, it's
// ok to keep them behind a single mutex.

/// A document database that allows clients to publish documents and
/// search for documents containing specific words.
pub struct Database {
    /// A map from words to the set of documents that contain them
    reverse_index: ConcurrentMultiMap<String, usize>,
    /// A store of all documents in the database
    blob_store: Mutex<Vec<String>>,
}

const BUCKETS: usize = 128;

impl Database {
    // Create a new empty archive. The map should have `BUCKETS` buckets.
    pub fn new() -> Self {
        let cmm = ConcurrentMultiMap::new(BUCKETS);
        let docs = Mutex::new(Vec::new());
        Database {
            reverse_index: cmm,
            blob_store: docs,
        }
    }

    // Publish a document to the archive in three steps:
    // 1. Make a new unique identifier for the document
    // 2. Split the document into words and map each word to the document's identifier in the
    //    reverse index. For our purposes, using built-in String functionality to split on
    //    whitespace is sufficient. It is up to you whether to also perform transformations like
    //    converting to lowercase or removing numerals.
    // 3. Add the document to the blob store
    pub fn publish(&self, doc: String) -> usize {
        let doc_clone = doc.clone();
        let mut docs = self.blob_store.lock().unwrap();
        let id = docs.len();
        docs.push(doc_clone);
        drop(docs);
        for word in doc.split_whitespace() {
            self.reverse_index.set(word.to_lowercase(), id);
        }
        id
    }
    // Use the reverse index to get the set of documents that contain the given word.
    pub fn search(&self, word: &str) -> Vec<usize> {
        self.reverse_index.get(word)
    }
    // Retrieve the document with the given id from the blob store.
    // Return None if the given id is invalid.
    pub fn retrieve(&self, id: usize) -> Option<String> {
        let docs = self.blob_store.lock().unwrap();
        let doc = docs.get(id)?;
        Some(String::from(doc))
    }
}
