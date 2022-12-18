#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;
mod move_zeros_to_the_end;
mod vowel_count;
mod your_order_please;
mod rgb_to_hex_conversion;
mod help_the_bookseller;
mod perimeter_of_squares_in_a_rectangle;

fn main() {
    let result = perimeter_of_squares_in_a_rectangle::perimeter2(30);
    println!("Result: {:?}", result);
}
