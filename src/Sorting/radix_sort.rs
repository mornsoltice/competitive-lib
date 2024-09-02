pub fn radix_sort(arr: &mut [i32]) {
    let max = *arr.iter().max().unwrap_or(&0);
    let mut exp = 1;
    while max / exp > 0 {
        counting_sort(arr, exp);
        exp *= 10;
    }
}

fn counting_sort(arr: &mut [i32], exp: i32) {
    let mut output = vec![0; arr.len()];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        let index = (num / exp % 10) as usize;
        count[index] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        let index = (num / exp % 10) as usize;
        output[count[index] as usize - 1] = num;
        count[index] -= 1;
    }

    arr.copy_from_slice(&output);
}
