//https://www.codewars.com/kata/559a28007caad2ac4e000083
// Original
struct FibIter {
    last_val: u64,
    current_val: u64,
}

impl FibIter {
    fn new() -> Self {
        Self { last_val: 1, current_val: 0 }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.last_val + self.current_val;
        self.last_val = self.current_val;
        self.current_val = next;
        Some(next)
    }
}

pub fn perimeter(n: u64) -> u64 {
    FibIter::new().take(n as usize + 1).sum::<u64>() * 4
}

// Better
pub fn perimeter2(n: u64) -> u64 {
    let fib_series = (0..=n).scan((0, 1), |prev, _| {
        let (x, y) = *prev;
        *prev = (y, x + y);
        Some(y)
    });
    4 * fib_series.sum::<u64>()
}
