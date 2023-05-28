mod document;
mod index;

use document::Document;

fn main() {
    let document1: Document = Document::new(
        String::from("I love rust and python they are so useful")
    );

    let document2: Document = Document::new(String::from("Loves Flutter and Java"));

    let document3: Document = Document::new(String::from("Loves Python and C++"));

    println!("Document 1: {:?}", document1);
    println!("Document 2: {:?}", document2);
    println!("Document 3: {:?}", document3);

    let mut index: index::Index = index::Index::new();

    index.add_document(document1);
    index.add_document(document2);
    index.add_document(document3);

    println!("Index: {:?}", index);

    let query: &str = "loves python";

    println!("Search for \"{}\": {:?}", query, index.search(query));
}
