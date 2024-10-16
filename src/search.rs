use std::collections::HashMap;

fn tfidf_vectorize(documents: &[&str]) -> Vec<Vec<f64>> {
    let mut term_freq = HashMap::new();
    let doc_count = documents.len() as f64;

    for doc in documents {
        let terms: Vec<&str> = doc.split_whitespace().collect();
        let unique_terms: HashMap<_, _> = terms.iter().cloned().map(|a| {
            (a, 0)
        }).collect();        
        for term in unique_terms.keys() {
            *term_freq.entry(*term).or_insert(0.0) += 1.0;
        }
    }

    let mut idf = HashMap::new();
    for (term, count) in term_freq.iter() {
        idf.insert(*term, (doc_count / *count).log(10.0)); // Use log to calculate IDF
    }

    let mut tfidf_matrix = vec![vec![0.0; term_freq.len()]; documents.len()];
    for (doc_idx, doc) in documents.iter().enumerate() {
        let terms: Vec<&str> = doc.split_whitespace().collect();
        let term_count = terms.len() as f64;
        let mut term_freq = HashMap::new();

        for term in terms {
            *term_freq.entry(term).or_insert(0.0) += 1.0;
        }

        for (i, (term, &count)) in term_freq.iter().enumerate() {
            if let Some(&idf_value) = idf.get(term) {
                tfidf_matrix[doc_idx][i] = (count / term_count) * idf_value;
            }
        }
    }
    
    tfidf_matrix
}

fn cosine_similarity(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut sim = vec![vec![0.0; matrix.len()]; matrix.len()];
    let norm: Vec<f64> = matrix.iter()
        .map(|row| row.iter().map(|&x| x.powi(2)).sum::<f64>().sqrt())
        .collect();

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            sim[i][j] = matrix[i].iter().zip(&matrix[j]).map(|(&a, &b)| a * b).sum();
        }
    }

    for i in 0..sim.len() {
        for j in 0..sim.len() {
            sim[i][j] /= norm[i] * norm[j];
        }
    }

    sim
}

fn cari_buku_mirip(judul_buku: &str, df: &[(String, String)], cosine_sim: &[Vec<f64>]) -> Vec<String> {
    let idx = df.iter().position(|(judul, _)| judul == judul_buku).unwrap();
    let sim_scores: Vec<(usize, f64)> = cosine_sim[idx]
        .iter()
        .enumerate()
        .map(|(i, &score)| (i, score))
        .collect();    

    let mut sorted_scores = sim_scores.clone();
    sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut result = Vec::new();
    for (i, score) in sorted_scores.iter().skip(1).take(5) {
        println!("{}: {}", i, score);
        result.push(df[*i].0.clone());
    }
    result
}

pub fn test() {
    let data_buku = vec![
        (String::from("Buku A"), String::from("Ini adalah deskripsi tentang Buku A.")),
        (String::from("Buku B"), String::from("Deskripsi Buku B berbicara tentang banyak hal menarik.")),
        (String::from("Buku C"), String::from("Buku C menjelaskan konsep-konsep dasar dalam ilmu pengetahuan.")),
        (String::from("Buku D"), String::from("Ini adalah deskripsi tentang Buku D.")),
    ];

    let documents: Vec<&str> = data_buku.iter().map(|(_, desc)| desc.as_str()).collect();
    
    let tfidf_matrix = tfidf_vectorize(&documents);
    let cosine_sim = cosine_similarity(&tfidf_matrix);

    let buku_mirip = cari_buku_mirip("Buku A", &data_buku, &cosine_sim);
    println!("Buku mirip dengan 'Buku A': {:?}", buku_mirip);
}

