pub fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let n: usize = lines[0].trim().parse().unwrap();

    if n == 0 {
        return "0".to_string();
    }

    // Read topics (beauty, difficulty)
    let mut topics = Vec::with_capacity(n);
    for line in lines.iter().skip(1).take(n) {
        let mut parts = line.split_whitespace();
        let beauty: i32 = parts.next().unwrap().parse().unwrap();
        let difficulty: i32 = parts.next().unwrap().parse().unwrap();
        topics.push((beauty, difficulty));
    }

    // Sort by beauty increasing, then difficulty decreasing
    topics.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    // Extract difficulties after sorting
    let difficulties: Vec<i32> = topics.iter().map(|&(_, diff)| diff).collect();

    // Find LIS of difficulties
    let result = lis_length(&difficulties);

    result.to_string()
}

/// Find the length of the Longest Increasing Subsequence (strictly increasing)
fn lis_length(arr: &[i32]) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut tails = Vec::new();

    for &num in arr {
        // Binary search for the position where num should be inserted
        let pos = tails.binary_search(&num).unwrap_or_else(|pos| pos);

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }

    tails.len()
}
