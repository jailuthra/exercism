use std::collections::HashSet;
use itertools::Itertools;

fn is_anagram(word: &str, word_lc: &str, sorted_word: &str, ana: &str) -> bool {
    if word.len() != ana.len() {
        return false;
    }
    let ana_lc = ana.to_lowercase();
    if word_lc == ana_lc {
        return false;
    }
    let sorted_ana  = ana_lc.chars().sorted().collect::<String>();
    sorted_word == sorted_ana
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_lc = word.to_lowercase();
    let sorted_word = word_lc.chars().sorted().collect::<String>();
    for &ana in possible_anagrams {
        if is_anagram(word, &word_lc, &sorted_word, ana) {
            anagrams.insert(ana);
        }
    }
    anagrams
}
