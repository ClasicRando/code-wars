// https://www.codewars.com/kata/513e08acc600c94f01000001
// Original
pub fn rgb(r: i32, g: i32, b: i32) -> String {
    let digit_to_hex = |d| {
        let to_hex = |n: i32| {
            if n < 10 {
                n.to_string()
            } else {
                match n {
                    10 => String::from("A"),
                    11 => String::from("B"),
                    12 => String::from("C"),
                    13 => String::from("D"),
                    14 => String::from("E"),
                    15 => String::from("F"),
                    _ => panic!("n should never be greater than 15"),
                }
            }
        };
        let remainder = d % 16;
        let quotient = (d - remainder) / 16;
        format!("{}{}", to_hex(quotient), to_hex(remainder))
    };
    format!("{}{}{}", digit_to_hex(r), digit_to_hex(g), digit_to_hex(b))
}

// Improved
pub fn rgb2(r: i32, g: i32, b: i32) -> String {
    let r = match r {
        0..=255 => r,
        _ if r > 255 => 255,
        _ => 0, 
    };
    let g = match g {
        0..=255 => g,
        _ if g > 255 => 255,
        _ => 0, 
    };
    let b = match b {
        0..=255 => b,
        _ if b > 255 => 255,
        _ => 0, 
    };
    format!("{:02X}{:02X}{:02X}", r, g, b)
}
  