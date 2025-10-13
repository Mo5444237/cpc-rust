use challenges::ch4::rle;

#[test]
fn test_run_length_encoding() {
    assert_eq!(rle("aaabbc"), vec![('a', 3), ('b', 2), ('c', 1)]);
    assert_eq!(rle("abc"), vec![('a', 1), ('b', 1), ('c', 1)]);
    assert_eq!(rle("aaaaa"), vec![('a', 5)]);
    assert_eq!(rle("aaaabbbaaa"), vec![('a', 4), ('b', 3), ('a', 3)]);
    assert_eq!(rle(""), Vec::<(char, usize)>::new());
}
