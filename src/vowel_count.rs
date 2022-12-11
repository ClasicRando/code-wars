// https://www.codewars.com/kata/54ff3102c1bad923760001f3
// Original
pub fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
}

// Better
pub fn get_count2(string: &str) -> usize {
    string
        .chars()
        .filter(|c| matches!(c, 'a'|'e'|'i'|'o'|'u'))
        .count()
}
