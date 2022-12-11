#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;
mod move_zeros_to_the_end;
mod vowel_count;
mod your_order_please;

fn main() {
    let result = move_zeros_to_the_end::move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]);
    println!("Result: {:?}", result);
}
