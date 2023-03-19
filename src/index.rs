use std::collections::{HashMap, LinkedList};

pub type DocumentID = i64;

#[derive(Debug, PartialEq, Eq)]
pub struct Index {
    pub dictionary: HashMap<String, PostingList>,
    pub total_docs_size: i32,
}

impl Index {
    pub fn new() -> Self {
        let dictionary: HashMap<String, PostingList> = HashMap::new();
        Index {
            dictionary,
            total_docs_size: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Posting {
    doc_id: DocumentID,
    positions: Vec<i32>,
    term_frequency: i32,
}

impl Posting {
    pub fn new(doc_id: DocumentID, positions: Vec<i32>) -> Self {
        Posting {
            doc_id,
            positions: positions.clone(),
            term_frequency: positions.len() as i32,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PostingList {
    list: LinkedList<Posting>,
}

impl PostingList {
    pub fn new(postings: Vec<Posting>) -> Self {
        let mut list: LinkedList<Posting> = LinkedList::new();
        for posting in postings {
            list.push_back(posting.clone());
        }
        Self { list }
    }

    pub fn add(&mut self, new: Posting) {
        if let Some(last) = self.list.back_mut() {
            if last.doc_id != new.doc_id {
                self.list.push_back(new);
                return;
            }
            last.positions.extend_from_slice(&new.positions);
            last.term_frequency += 1;
        } else {
            self.list.push_back(new);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_test() {
        let index = Index::new();
        assert_eq!(0, index.total_docs_size);
    }

    #[test]
    fn positions_list_test() {
        let mut pos_list = PostingList::new((&[]).to_vec());
        let pos1 = Posting::new(1, vec![1, 2, 3]);
        let pos2 = Posting::new(2, vec![4, 5]);
        pos_list.add(pos1);
        pos_list.add(pos2);

        let mut iter = pos_list.list.iter();

        assert_eq!(
            iter.next().unwrap(),
            &Posting {
                doc_id: 1,
                positions: vec![1, 2, 3],
                term_frequency: 3
            }
        );
        assert_eq!(
            iter.next().unwrap(),
            &Posting {
                doc_id: 2,
                positions: vec![4, 5],
                term_frequency: 2
            }
        );
    }
}
