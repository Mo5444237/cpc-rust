use challenges::ch1::max_depth;

#[test]
fn test_max_depth() {
    assert_eq!(max_depth("((()))"), 3);
    assert_eq!(max_depth("(a(b(c)d)e)"), 3);
    assert_eq!(max_depth("abc"), 0);
    assert_eq!(max_depth("( a(b) (c) (d(e(f)g)h) I (j(k)l)m)"), 4);
}
