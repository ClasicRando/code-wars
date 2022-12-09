#[allow(dead_code)]
// https://www.codewars.com/kata/5544c7a5cb454edb3c000047

// Original
pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0.0 || !(0.0..1.0).contains(&bounce) || window >= h {
        return -1;
    }
    let mut seen = 1;
    let mut current_height = h * bounce;
    while current_height > window {
        seen += 2;
        current_height *= bounce;
    }
    seen
}

// Better
///////////////////////////////////////////////////////////////////////////////////////////////////
/// First peak is h, second peak is h * bounce, thrid peak is h * bounce * bounce, ...
/// Ball will still be seen from window while peak height > window
/// 
/// Peak height as a function of bounces (x)
/// 
///     peak height = h (b ^ x)
/// 
/// Therefore the max bounces seen is when window = peak height
/// 
///     window = peak height = h (b ^ x)
/// 
/// Solve for x
/// 
///     window / h = b ^ x
///     log_b(window / h) = log_b(b ^ x)
///     log_b(window / h) = x * log_b(b)
///     x = log_b(window / h)
/// 
/// Since this result can be fractional, we round up, multiply by 2 (since each peak is seen twice)
/// and subtract 1 (since the first peak height is only seen once).
pub fn bouncing_ball2(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0.0 || !(0.0..1.0).contains(&bounce) || window >= h {
        return -1;
    }
    ((window / h).log(bounce).ceil() as i32 * 2) - 1
}
