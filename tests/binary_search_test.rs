use competitive_lib::Algorithms::binary_search;

#[test]
fn test_binary_search() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(binary_search::binary_search(&arr, 3), Some(2));
    assert_eq!(binary_search::binary_search(&arr, 6), None);
}
