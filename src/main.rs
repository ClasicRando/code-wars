#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;
mod move_zeros_to_the_end;
mod vowel_count;
mod your_order_please;
mod rgb_to_hex_conversion;
mod help_the_bookseller;

fn main() {
    let result = help_the_bookseller::stock_list(
        vec![],
        vec!["B", "R", "D", "X"],
    );
    println!("Result: {:?}", result);
}
