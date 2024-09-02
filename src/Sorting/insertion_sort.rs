pub fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i as isize - 1;
        while j >= 0 && arr[j as usize] > key {
            arr[j as usize + 1] = arr[j as usize];
            j -= 1;
        }
        arr[j as usize + 1] = key;
    }
}
