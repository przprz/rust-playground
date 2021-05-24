pub(crate) fn a_closure() {
    println!("a_closure");

    let range = 0..10;
    let cnt = || range.count();
    assert_eq!(cnt(), 10);
    // cnt(); // won't compile
}
