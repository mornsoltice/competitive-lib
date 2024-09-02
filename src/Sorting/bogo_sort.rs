use rand::seq::SliceRandom;

pub fn bogo_sort(arr: &mut [i32]) {
    let mut rng = rand::thread_rng();
    while !is_sorted(arr) {
        arr.shuffle(&mut rng);
    }
}

fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}
