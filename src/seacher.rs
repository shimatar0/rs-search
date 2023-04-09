use std::collections::LinkedList;

use crate::{
    index::{DocumentID, Posting},
    index_reader::IndexReader,
};

pub struct TopDocs {
    total_hits: i32,
    score_docs: Vec<ScoreDoc>,
}

#[derive(Debug)]
pub struct ScoreDoc {
    doc_id: DocumentID,
    score: f64,
}

pub struct Searcher {
    index_reader: IndexReader,
    cursor: Vec<LinkedList<Posting>>,
}

#[derive(Debug)]
struct CalcIFIDF {
    term_freq: i32,
    doc_count: i32,
}

impl Searcher {
    pub fn new(path: String) -> Self {
        return Searcher {
            index_reader: IndexReader::new(path),
            cursor: Vec::new(),
        };
    }

    pub fn search(&mut self, query: Vec<String>) -> Vec<ScoreDoc> {
        let score_doc: Vec<ScoreDoc> = Vec::new();

        if Searcher::open_cursors(self, query) == 0 {
            return score_doc;
        }

        let mut docs: Vec<ScoreDoc> = Vec::new();

        let posting_iter = self.cursor.clone();
        let mut front_iter = posting_iter[0].iter().peekable();
        let other_iter = posting_iter[1..].to_vec();

        while let Some(front) = front_iter.next() {
            let mut next_doc_id: DocumentID = 0;
            let mut calc_doc_list: Vec<CalcIFIDF> = Vec::new();

            for other in other_iter.clone() {
                let mut list_iter = other.iter().peekable();
                let mut current = None;
                while let Some(item) = list_iter.peek().cloned() {
                    current = Some(item);
                    list_iter.next();
                    if item.doc_id >= front.doc_id {
                        break;
                    }
                }

                if let Some(c) = current {
                    if c.doc_id != front.doc_id {
                        next_doc_id = c.doc_id;
                        calc_doc_list.clear();
                        break;
                    } else {
                        let calc_data = CalcIFIDF {
                            term_freq: current.unwrap().term_frequency,
                            doc_count: other.len() as i32,
                        };
                        let front_calc_data = CalcIFIDF {
                            term_freq: front.term_frequency,
                            doc_count: self.cursor[0].len() as i32,
                        };
                        calc_doc_list.push(calc_data);
                        calc_doc_list.push(front_calc_data);
                    }
                } else {
                    return docs;
                }
            }
            if next_doc_id > 0 {
                while let Some(posting) = front_iter.next() {
                    if posting.doc_id >= next_doc_id {
                        break;
                    }
                }
            } else {
                let score_doc: ScoreDoc = ScoreDoc {
                    doc_id: front.doc_id,
                    score: self.calc_score(calc_doc_list),
                };
                docs.push(score_doc);
            }
        }
        docs
    }

    pub fn open_cursors(&mut self, query: Vec<String>) -> i32 {
        let mut postings = self.index_reader.postings_list(query);
        if postings.is_empty() {
            return 0;
        }
        postings.sort_by_key(|pl| pl.list.len());

        let mut cursors: Vec<LinkedList<Posting>> = Vec::with_capacity(postings.len());
        // get iter≒cursor
        for posting_list in postings {
            cursors.push(posting_list.list);
        }
        let len = cursors.len().clone() as i32;
        self.cursor = cursors;
        len
    }

    pub fn search_top_k(mut self, query: Vec<String>, k: i32) -> TopDocs {
        let result = self.search(query);

        return TopDocs {
            total_hits: result.len() as i32,
            score_docs: result,
        };
    }

    fn calc_score(&self, doc_list: Vec<CalcIFIDF>) -> f64 {
        let mut score: f64 = 0.0;
        for doc in doc_list {
            let term_freq = doc.term_freq;
            let doc_count = doc.doc_count;
            let total_doc_cnt = 10;
            score += self.calc_tf(term_freq) + self.calc_idf(total_doc_cnt, doc_count);
        }
        score
    }

    fn calc_tf(&self, term_cnt: i32) -> f64 {
        if term_cnt <= 0 {
            return 0.0;
        }
        return f64::log2(term_cnt as f64) + 1.0;
    }

    fn calc_idf(&self, n: i32, df: i32) -> f64 {
        return f64::log2(n as f64) / df as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::Searcher;

    #[test]
    fn searcher() {
        let mut s = Searcher::new("./_index_data".to_string());
        let q = vec!["do".to_string(), "sir".to_string()];
        let docs = s.search(q);
        println!("{:?}", docs);
    }
}
