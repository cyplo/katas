extern crate core;
use core::iter::FromIterator;

pub fn anagrams_for<'a>(base_string: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let base_length = base_string.len();
    let filtered_candidates = candidates.iter()
        .filter(|candidate| candidate.len() == base_length)
        .filter(|candidate| is_not_same_word(base_string, candidate))
        .filter(|candidate| has_all_letters(base_string, candidate))
        .map(|&candidate| candidate);
    Vec::from_iter(filtered_candidates)
}

fn is_not_same_word(lhs: &str, rhs: &str) -> bool {
    lhs != rhs
}

fn has_all_letters(base: &str, candidate: &str) -> bool {
    let mut available_letters = Vec::from_iter(candidate.chars());
    for letter in base.chars() {
        let position = available_letters.iter()
            .position(|&tested_letter| tested_letter == letter);

        if position.is_none() {
            return false;
        }

        available_letters.remove(position.unwrap());
    }
    true
}
