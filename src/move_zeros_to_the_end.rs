// https://www.codewars.com/kata/52597aa56021e91c93000cb0
// Original
pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut nums = Vec::with_capacity(arr.len());
    let mut zero_count = 0;
    for n in arr.iter() {
        if *n != 0 {
            nums.push(*n)
        } else {
            zero_count += 1;
        }
    }
    nums.extend(vec![0; zero_count]);
    nums
}

// Better
pub fn move_zeros2(arr: &[u8]) -> Vec<u8> {
    let mut nums = Vec::with_capacity(arr.len());
    nums.extend(arr.iter().filter(|&&n| n != 0));
    nums.resize(arr.len(), 0);
    nums
}
