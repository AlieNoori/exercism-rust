use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut words = HashSet::new();

    let lower_word = word.to_lowercase();
    let normalized_word = normalize_word(&lower_word);

    for &candidate in possible_anagrams {
        let lower_candidate = candidate.to_lowercase();

        if lower_candidate == lower_word {
            continue;
        }

        let normalized_candidate = normalize_word(&lower_candidate);

        if normalized_candidate == normalized_word {
            words.insert(candidate);
        }
    }

    words
}

fn normalize_word(word: &str) -> String {
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort_unstable();
    word_chars.into_iter().collect()
}
