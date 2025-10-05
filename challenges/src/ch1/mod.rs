pub fn max_depth(s: &str) -> usize {
    s.chars()
        .fold((0usize, 0usize), |(curr, max), ch| match ch {
            '(' => {
                let curr = curr + 1;
                (curr, max.max(curr))
            }
            ')' => (curr.saturating_sub(1), max),
            _ => (curr, max),
        })
        .1
}
