pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as isize - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid as usize] == target {
            return Some(mid as usize);
        } else if arr[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}
