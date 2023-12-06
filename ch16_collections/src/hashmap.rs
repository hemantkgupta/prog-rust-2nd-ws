/* 
use std::collections::{HashMap, HashSet};
use std::fs::File;

pub fn hashmap_work(){
    let mut vote_counts: HashMap<String, usize> = HashMap::new();
    let ballots = ["".to_string(), "".to_string()];
    for name in ballots {
        let count = vote_counts.entry(name).or_insert(0);
        *count += 1;
    }

    // This map contains, for each word, the set of files it appears in.
    let mut word_occurrence: HashMap<String, HashSet<String>> =
    HashMap::new();
    let files = vec![];
    for file in files {
        for word in read_words(file)? {
            let set = word_occurrence
                .entry(word)
                .or_insert_with(HashSet::new);
            set.insert(file.clone());
        }
    }

    // This map contains all the words in a given string,
    // along with the number of times they occur.
    let mut word_frequency: HashMap<&str, u32> = HashMap::new();
    for c in text.split_whitespace() {
        word_frequency.entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }



}
*/