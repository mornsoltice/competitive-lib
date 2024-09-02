use competitive_programming_lib::Sorting::bogo_sort::bogo_sort;
use competitive_programming_lib::Sorting::bubble_sort::bubble_sort;
use competitive_programming_lib::Sorting::insertion_sort::insertion_sort;
use competitive_programming_lib::Sorting::merge_sort::merge_sort;
use competitive_programming_lib::Sorting::quick_sort::quick_sort;
use competitive_programming_lib::Sorting::radix_sort::radix_sort;
use competitive_programming_lib::Sorting::selection_sort::selection_sort;

#[test]
fn test_bubble_sort() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut arr);
    assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
}

#[test]
fn test_quick_sort() {
    let mut arr = [10, 7, 8, 9, 1, 5];
    quick_sort(&mut arr);
    assert_eq!(arr, [1, 5, 7, 8, 9, 10]);
}

#[test]
fn test_radix_sort() {
    let mut arr = [170, 45, 75, 90, 802, 24, 2, 66];
    radix_sort(&mut arr);
    assert_eq!(arr, [2, 24, 45, 66, 75, 90, 170, 802]);
}

#[test]
fn test_bogo_sort() {
    let mut arr = [3, 2, 5, 1, 4];
    bogo_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn test_merge_sort() {
    let mut arr = [12, 11, 13, 5, 6, 7];
    merge_sort(&mut arr);
    assert_eq!(arr, [5, 6, 7, 11, 12, 13]);
}

#[test]
fn test_insertion_sort() {
    let mut arr = [12, 11, 13, 5, 6];
    insertion_sort(&mut arr);
    assert_eq!(arr, [5, 6, 11, 12, 13]);
}

#[test]
fn test_selection_sort() {
    let mut arr = [64, 25, 12, 22, 11];
    selection_sort(&mut arr);
    assert_eq!(arr, [11, 12, 22, 25, 64]);
}
