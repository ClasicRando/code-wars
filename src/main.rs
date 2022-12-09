#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;

fn main() {
    let result = decode_morse_code::decode_morse2(".... . -.--   .--- ..- -.. .");
    println!("Result: {}", result);
}
