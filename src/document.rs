use std::sync::atomic::{ AtomicUsize, Ordering };

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

const STOP_WORDS: &str = include_str!("./stop_words.txt");

#[derive(Debug)]
pub struct Document {
    pub id: usize,
    pub text: String,
}

impl Document {
    pub fn new(text: String) -> Self {
        Document {
            id: OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst),
            text,
        }
    }

    pub fn tokenize(&self) -> Vec<String> {
        let lowercase_text = self.text.to_lowercase();
        let words: Vec<&str> = lowercase_text.split_whitespace().collect();
        let tokens = words.iter().filter(|word| !STOP_WORDS.contains(&word.to_string()));

        tokens.map(|word| word.to_string()).collect()
    }
}
