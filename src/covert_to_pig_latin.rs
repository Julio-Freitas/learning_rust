pub fn convert_ping_latin(word: &String) -> String {
    let vowel = ["A", "e", "i", "o", "u"];
    let (first_letter, rest) = word.split_at(1);
    let start_with_vowel = vowel.contains(&first_letter);
    if start_with_vowel {
        return format!("{}-hay", word);
    }
    return format!("{}-{}-ay", rest, first_letter);
}
