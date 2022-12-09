use std::collections::HashMap;

// https://www.codewars.com/kata/54b724efac3d5402db00065e
fn morse_code() -> HashMap<String, String> {
    let mut morse = HashMap::new();
    morse.insert(String::from(".-"), String::from("A"));
    morse.insert(String::from("-..."), String::from("B"));
    morse.insert(String::from("-.-"), String::from("C"));
    morse.insert(String::from("-.."), String::from("D"));
    morse.insert(String::from("."), String::from("E"));
    morse.insert(String::from("..-."), String::from("F"));
    morse.insert(String::from("--."), String::from("G"));
    morse.insert(String::from("...."), String::from("H"));
    morse.insert(String::from(".."), String::from("I"));
    morse.insert(String::from(".---"), String::from("J"));
    morse.insert(String::from("-."), String::from("K"));
    morse.insert(String::from(".-.."), String::from("L"));
    morse.insert(String::from("--"), String::from("M"));
    morse.insert(String::from("-."), String::from("N"));
    morse.insert(String::from("---"), String::from("O"));
    morse.insert(String::from(".--."), String::from("P"));
    morse.insert(String::from("--.-"), String::from("Q"));
    morse.insert(String::from(".-."), String::from("R"));
    morse.insert(String::from("..."), String::from("S"));
    morse.insert(String::from("-"), String::from("T"));
    morse.insert(String::from("..-"), String::from("U"));
    morse.insert(String::from("...-"), String::from("V"));
    morse.insert(String::from(".--"), String::from("W"));
    morse.insert(String::from("-..-"), String::from("X"));
    morse.insert(String::from("-.--"), String::from("Y"));
    morse.insert(String::from("--.."), String::from("Z"));
    morse.insert(String::from(".----"), String::from("1"));
    morse.insert(String::from("..---"), String::from("2"));
    morse.insert(String::from("...--"), String::from("3"));
    morse.insert(String::from("....-"), String::from("4"));
    morse.insert(String::from("....."), String::from("5"));
    morse.insert(String::from("-...."), String::from("6"));
    morse.insert(String::from("--..."), String::from("7"));
    morse.insert(String::from("---.."), String::from("8"));
    morse.insert(String::from("----."), String::from("9"));
    morse.insert(String::from("-----"), String::from("0"));
    morse
}

// Original
pub fn decode_morse(encoded: &str) -> String {
    #[allow(non_snake_case)]
    let MORSE_CODE = morse_code();
    encoded
        .split("   ")
        .map(|word| {
            word.split(' ')
                .filter_map(|code| -> Option<&str> {
                    if code.trim() == "" {
                        return None
                    }
                    MORSE_CODE
                        .get(code.trim())
                        .map(|l| l.as_str())
                })
                .collect::<String>()
                + " "
        })
        .collect::<String>()
        .trim()
        .to_string()
}

// Better
pub fn decode_morse2(encoded: &str) -> String {
    #[allow(non_snake_case)]
    let MORSE_CODE = morse_code();
    encoded
        .split(' ')
        .map(|code| {
            MORSE_CODE
                .get(code)
                .map(|l| l.as_str())
                .unwrap_or(" ")
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
