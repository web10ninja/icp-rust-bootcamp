use std::collections::HashMap;

fn main() {
    // Sample text for analysis
    let text = "Rust is a multi-paradigm, general-purpose programming language. \
                Rust emphasizes performance, type safety, and concurrency. \
                Rust enforces memory safety—that is, that all references point \
                to valid memory—without requiring the use of a garbage collector \
                or reference counting present in other memory-safe languages.";

    // 1. Split the text into words and store them in a vector
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '-').to_string())
        .filter(|w| !w.is_empty())
        .collect();

    // 2. Count the frequency of each word and store in a HashMap
    let mut word_counts = HashMap::new();
    for word in &words {
        *word_counts.entry(word.clone()).or_insert(0) += 1;
    }

    // 3. Find the longest word in the text
    let longest_word = words.iter().max_by_key(|w| w.len()).unwrap_or(&String::new());

    // 4. Convert all words to uppercase and store in a new vector
    let uppercase_words: Vec<String> = words.iter().map(|w| w.to_uppercase()).collect();

    // 5. Filter out words shorter than 4 characters
    let filtered_words: Vec<&String> = words.iter().filter(|w| w.len() >= 4).collect();

    // Print results
    println!("Word counts: {:?}", word_counts);
    println!("Longest word: {}", longest_word);
    println!("Uppercase words: {:?}", uppercase_words);
    println!("Words longer than 3 characters: {:?}", filtered_words);
}