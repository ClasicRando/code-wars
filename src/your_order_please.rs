// https://www.codewars.com/kata/55c45be3b2079eccff00010f
// Original
pub fn order(sentence: &str) -> String {
    if sentence.is_empty() {
        return String::new();
    }
    let mut words = sentence
        .split_whitespace()
        .map(|word| {
            let num = word.chars().find_map(|c| if c.is_numeric() { c.to_digit(10) } else { None }).unwrap();
            (num, word)
        })
        .collect::<Vec<_>>();
    words.sort_by(|(num1, _), (num2, _)| num1.cmp(num2));
    words.into_iter().map(|(_, word)| word).collect::<Vec<_>>().join(" ")
}

// Better
pub fn order2(sentence: &str) -> String {
    let mut words = sentence.split_whitespace().collect::<Vec<_>>();
    words.sort_by_key(|w| w.chars().find(|c| c.is_ascii_digit()).unwrap());
    words.join(" ")
}
