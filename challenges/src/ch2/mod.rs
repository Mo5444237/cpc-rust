pub fn missing_number(a: &[u32]) -> u32 {
    let n = a.len() as u32;
    let all = (0..=n).fold(0, |x, i| x ^ i);
    let arr = a.iter().copied().fold(0, |x, i| x ^ i);
    all ^ arr
}
