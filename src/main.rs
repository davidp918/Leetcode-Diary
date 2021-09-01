pub mod links;

fn main() {
    // playground

    println!(
        "{}",
        Solution::ladder_length(
            "aa".to_string(),
            "bb".to_string(),
            vec!["ab".to_string(), "bb".to_string(),],
        )
    );
}

struct Solution {}

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        let (n, wordlen) = (word_list.len(), begin_word.len()); // wordlen must be uniform across all words
        let maskmap: HashMap<String, Vec<usize>> = Self::construct_dict(&word_list, wordlen);
        word_list.push(begin_word); // initialization
        let mut deque: VecDeque<(usize, i32)> = VecDeque::from(vec![(n, 1)]);
        let mut seen: Vec<bool> = vec![false; n + 1]; // indicating whether word_list[i] is visited

        while let Some((cur, step)) = deque.pop_front() {
            let word = &word_list[cur];
            if word == &end_word {
                return step;
            }

            // for every word, push all the possible nexts that
            // differs by 1 letter using the pre-computed hashmap
            for i in 0..wordlen {
                let mask = format!("{}_{}", &word[..i], &word[i + 1..]);
                if let Some(nexts) = maskmap.get(&mask) {
                    for &next in nexts {
                        if seen[next] == false {
                            seen[next] = true;
                            deque.push_back((next, step + 1));
                        }
                    }
                }
            }
        }

        0
    }

    // construct a hashmap with masks as keys, and values which are possible routes
    // to the end_word, while storing words using index i as word_list[i]
    fn construct_dict(word_list: &Vec<String>, length: usize) -> HashMap<String, Vec<usize>> {
        let mut maskmap: HashMap<String, Vec<usize>> = HashMap::new();

        for i in 0..word_list.len() {
            for j in 0..length {
                let word = &word_list[i];
                let mask = format!("{}_{}", &word[..j], &word[j + 1..]);
                maskmap.entry(mask).or_insert(vec![]).push(i);
            }
        }

        maskmap
    }
}
