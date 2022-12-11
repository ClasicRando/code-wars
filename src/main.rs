#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;
mod decode_morse_code;
mod vowel_count;
mod your_order_please;

fn main() {
    let result = your_order_please::order2("4of Fo1r pe6ople g3ood th5e the2");
    println!("Result: {}", result);
}
