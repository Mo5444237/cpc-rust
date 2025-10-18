use challenges::ch2::missing_number;

#[test]
fn test_missing_number() {
    assert_eq!(missing_number(&[3, 0, 1]), 2);
    assert_eq!(missing_number(&[0, 1, 2, 4]), 3);
    assert_eq!(missing_number(&[5, 2, 0, 1, 3]), 4);
    assert_eq!(missing_number(&[1]), 0);
}

#[test]
fn test_missing_number_edge_cases() {
    assert_eq!(missing_number(&[]), 0); // from 0..=0, missing is 0
    assert_eq!(missing_number(&[0]), 1); // from 0..=1, missing is 1
}
