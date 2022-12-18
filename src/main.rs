#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;
mod move_zeros_to_the_end;
mod vowel_count;
mod your_order_please;
mod rgb_to_hex_conversion;

fn main() {
    let result = rgb_to_hex_conversion::rgb2(-20, 275, 125);
    println!("Result: {:?}", result);
}
