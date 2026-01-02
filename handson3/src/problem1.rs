pub fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut first_line = lines[0].split_whitespace();
    let n = first_line.next().unwrap().parse::<usize>().unwrap();
    let d = first_line.next().unwrap().parse::<usize>().unwrap();

    // Read cities' attractions
    let mut cities = Vec::with_capacity(n);
    for line in lines.iter().skip(1).take(n) {
        let attractions: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cities.push(attractions);
    }

    // Precompute cumulative attractions
    let mut cum = vec![vec![0; d + 1]; n + 1];
    for i in 1..=n {
        let city = &cities[i - 1];
        for day in 1..=d {
            cum[i][day] = if day <= city.len() {
                cum[i][day - 1] + city[day - 1]
            } else {
                cum[i][day - 1]
            };
        }
    }

    // dp[i][j] = max attractions using first i cities with j days
    let mut dp = vec![vec![0; d + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=d {
            for k in 0..=j {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - k] + cum[i][k]);
            }
        }
    }

    dp[n][d].to_string()
}
