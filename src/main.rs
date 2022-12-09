#![allow(dead_code)]

mod bit_counting;
mod bouncing_balls;

fn main() {
    let result = bit_counting::count_bits(7);
    println!("Result: {}", result);
}
