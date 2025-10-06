pub fn majority(a: &[u32]) -> Option<u32> {
    let candidate = a
        .iter()
        .fold((0, 0), |(cand, count), &x| {
            if count == 0 {
                (x, 1)
            } else if x == cand {
                (cand, count + 1)
            } else {
                (cand, count - 1)
            }
        })
        .0;

    (a.iter().filter(|&&x| x == candidate).count() > a.len() / 2).then_some(candidate)
}
