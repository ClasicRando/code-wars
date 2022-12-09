#![allow(dead_code)]

mod bouncing_balls;

fn main() {
    let result = bouncing_balls::bouncing_ball(30.0, 0.66, 1.5);
    println!("Result: {}", result);
}
