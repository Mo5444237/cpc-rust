use itertools::Itertools;

pub fn rle(s: &str) -> Vec<(char, usize)> {
    s.chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(ch, group)| (ch, group.count()))
        .collect()
}
