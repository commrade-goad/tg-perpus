use crate::book::{self, sample_books};
use std::collections::HashMap;

fn vectorize_book(documents: Vec<book::Book>) -> Vec<HashMap<String, i32>> {
    let mut all_word_count: Vec<HashMap<String, i32>> = Vec::new();
    for doc in documents {
        let mut word_count = HashMap::new();
        for word in doc.title.clone().split_whitespace() {
            *word_count.entry(word.to_lowercase().to_string()).or_insert(0) += 1;
        }
        for word in doc.author.clone().split_whitespace() {
            *word_count.entry(word.to_lowercase().to_string()).or_insert(0) += 1;
        }
        for word in doc.tags.clone() {
            *word_count.entry(word.to_lowercase().to_string()).or_insert(0) += 1;
        }
        all_word_count.push(word_count);
    }

    let mut result: Vec<HashMap<String, i32>> = Vec::new();

    for i in 0..all_word_count.len() {
        result.push(all_word_count[i].clone());
        if i < all_word_count.len() - 1 {
            let tmp = all_word_count[i+1].clone();
            for (key, val) in tmp{
                let _ = *result[i].entry(key).or_insert(0);
            }
        } else {
            let tmp = all_word_count[i-1].clone();
            for (key, val) in tmp{
                let _ = *result[i].entry(key).or_insert(0);
            }
        }
    }


    return result;
}

pub fn test() -> () {
    let book: Vec<book::Book> = sample_books();
    let stuff = vectorize_book(book);
    println!("{:?}", stuff);
}
