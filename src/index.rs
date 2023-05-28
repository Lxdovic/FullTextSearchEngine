use std::collections::{ HashMap };
use crate::document::Document;

#[derive(Debug)]
pub struct Index {
    inverted_index: HashMap<String, Vec<usize>>,
    documents: Vec<Document>,
}

#[derive(Debug)]
pub struct Result<'a> {
    pub document: &'a Document,
    pub score: f32,
}

impl Index {
    pub fn new() -> Index {
        Index {
            inverted_index: HashMap::new(),
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, document: Document) {
        let tokens = document.tokenize();

        for token in tokens {
            let entry = self.inverted_index.entry(token).or_insert(Vec::new());

            entry.push(document.id);
        }

        self.documents.push(document);
    }

    pub fn search(&self, query: &str) -> Vec<Result> {
        let lowercase_query = query.to_lowercase();
        let splitted_query = lowercase_query.split_whitespace().collect::<Vec<&str>>();

        let mut result: Vec<Result> = Vec::new();

        for word in splitted_query {
            if let Some(entry) = self.inverted_index.get(word) {
                for id in entry {
                    let document = &self.documents[*id];
                    let mut score: f32 = 0.0;

                    let index: Option<usize> = document.text
                        .to_lowercase()
                        .find(&word)
                        .map(|i| i + 1);

                    println!("{:?}: {}: {}", index, word, document.text);

                    match index {
                        Some(i) => {
                            score += 1.0 / (i as f32);
                        }
                        None => (),
                    }

                    result.push(Result {
                        document: &self.documents[*id],
                        score,
                    });
                }
            }
        }

        result
    }
}
