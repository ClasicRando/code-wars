#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;
mod vowel_count;

fn main() {
    let result = vowel_count::get_count("abracadabra");
    println!("Result: {}", result);
}
