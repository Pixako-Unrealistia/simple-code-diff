use std::fs;
use std::collections::{HashSet};

fn main() {
    let dir = "submissions";
    let threshold = 0.7;
    let mut submissions: Vec<(String, HashSet<String>)> = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let content = fs::read_to_string(&path).unwrap();
        let words: HashSet<String> = content.split_whitespace().map(|s| s.to_string()).collect();

        submissions.push((filename.to_string(), words));
    }

    let mut groups: Vec<Vec<String>> = Vec::new();

    for i in 0..submissions.len() {
        let (filename1, words1) = &submissions[i];
        let mut group: Vec<String> = Vec::new();
        group.push(filename1.to_string());

        for j in i+1..submissions.len() {
            let (filename2, words2) = &submissions[j];
            let intersection = words1.intersection(words2).count();
            let union = words1.union(words2).count();
            let jaccard_similarity = intersection as f64 / union as f64;

            if jaccard_similarity >= threshold {
                group.push(filename2.to_string());
            }
        }

        if group.len() > 1 {
            groups.push(group);
        }
    }

    for group in groups {
        println!("Possible cheaters: {:?}", group);
    }
}
