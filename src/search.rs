use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::book;
use crate::sql;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
struct SortedData {
    index: i32,
    pub score: f64,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    book: book::Book,
    score: f64,
}

fn vectorize_book(documents: &Vec<book::Book>) -> Vec<HashMap<String, f64>> {
    let mut all_word_count: Vec<HashMap<String, f64>> = Vec::new();
    for doc in documents {
        let mut word_count = HashMap::new();
        for word in doc.title.clone().split_whitespace() {
            *word_count
                .entry(word.to_lowercase().to_string())
                .or_insert(0.0) += 1.0;
        }
        for word in doc.author.clone().split_whitespace() {
            *word_count
                .entry(word.to_lowercase().to_string())
                .or_insert(0.0) += 1.0;
        }
        for word in doc.tags.clone() {
            *word_count
                .entry(word.name.to_lowercase().to_string())
                .or_insert(0.0) += 1.0;
        }
        *word_count.entry(doc.year.to_string()).or_insert(0.0) += 1.0;
        all_word_count.push(word_count);
    }

    let mut result: Vec<HashMap<String, f64>> = Vec::new();

    for i in 0..all_word_count.len() {
        result.push(all_word_count[i].clone());
        if i < all_word_count.len() - 1 {
            let tmp = all_word_count[i + 1].clone();
            for (key, _) in tmp {
                let _ = *result[i].entry(key).or_insert(0.0);
            }
        } else {
            let tmp = all_word_count[0].clone();
            for (key, _) in tmp {
                let _ = *result[i].entry(key).or_insert(0.0);
            }
        }
    }

    return result;
}

fn vectorize_word(words: &str, vector_book: Vec<HashMap<String, f64>>) -> HashMap<String, f64> {
    let mut result: HashMap<String, f64> = HashMap::new();
    let keywords: Vec<String> = words.split_whitespace().map(|w| w.to_lowercase()).collect();

    for w in keywords {
        *result.entry(w.clone()).or_insert(0.0) += 1.0;

        for obj in &vector_book {
            for (key, _) in obj {
                // if key.contains(&w) && w.len() >= 2{
                if key.contains(&w) {
                    *result.entry(key.clone()).or_insert(0.0) += 1.0;
                }
            }
        }
    }

    result
}

fn cosine_similarity(vec1: &HashMap<String, f64>, vec2: &HashMap<String, f64>) -> f64 {
    let dot_product: f64 = vec1
        .iter()
        .filter_map(|(k, v1)| vec2.get(k).map(|v2| v1 * v2))
        .sum();

    let magnitude1: f64 = vec1.values().map(|v| v * v).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.values().map(|v| v * v).sum::<f64>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude1 * magnitude2)
}

pub async fn s_search_book(keyword: &str, sort_mode: String) -> Vec<SearchResult> {
    let keyword_str: String = keyword.to_string();
    let mut result: Vec<SearchResult> = Vec::new();
    tokio::task::spawn_blocking(move || {
        let book: Vec<book::Book> = sql::sql_read_book(sort_mode).unwrap();
        let stuff = vectorize_book(&book);
        let stuff2 = vectorize_word(&keyword_str, stuff.clone());
        let mut kesamaan: Vec<SortedData> = Vec::new();
        for i in 0..stuff.len() {
            let obj = &stuff[i];
            kesamaan.push(SortedData {
                index: i as i32,
                score: cosine_similarity(&stuff2, &obj),
            })
        }
        kesamaan.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for k in &kesamaan {
            if k.score > 0.0 {
                let new_obj: SearchResult = SearchResult {
                    book: book[k.index as usize].clone(),
                    score: k.score.clone(),
                };
                result.push(new_obj);
            }
        }
        return result;
    })
    .await
    .unwrap()
}
