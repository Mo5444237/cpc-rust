use handson3::problem2::solve;

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n").trim_end().to_string()
}

fn load_case(i: usize) -> (String, String) {
    let input_path = format!("data/problem2/input{}.txt", i);
    let output_path = format!("data/problem2/output{}.txt", i);

    let input = std::fs::read_to_string(std::path::Path::new("tests").join(&input_path))
        .unwrap_or_else(|_| panic!("Cannot read {}", input_path));

    let expected = std::fs::read_to_string(std::path::Path::new("tests").join(&output_path))
        .unwrap_or_else(|_| panic!("Cannot read {}", output_path));

    (input, expected)
}

#[test]
fn test_all_problem2_cases() {
    for i in 0..=9 {
        let (input, expected) = load_case(i);
        let got = solve(&input);

        assert_eq!(
            normalize(&got),
            normalize(&expected),
            "Mismatch in Problem 2 case {}",
            i
        );
    }
}
