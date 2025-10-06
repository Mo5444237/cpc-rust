use ::challenges::ch3::majority;

#[test]
fn test_majority() {
    assert_eq!(majority(&[3, 2, 3]), Some(3));
    assert_eq!(majority(&[2, 2, 1, 1, 1, 2, 2]), Some(2));
    assert_eq!(majority(&[1, 2, 3, 4]), None);
    assert_eq!(majority(&[1, 1, 1, 2, 2]), Some(1));
    assert_eq!(majority(&[1]), Some(1));
}

#[test]
fn test_majority_edge_cases() {
    assert_eq!(majority(&[]), None); 
    assert_eq!(majority(&[1, 2]), None); 
    assert_eq!(majority(&[1, 1, 2, 2, 2]), Some(2));
}
