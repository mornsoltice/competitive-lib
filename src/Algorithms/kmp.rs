pub fn kmp_pattern_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut lps = vec![0; pattern.len()];
    let mut j = 0;
    compute_lps(pattern, &mut lps);

    let mut matches = Vec::new();
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let mut i = 0;

    while i < text.len() {
        if pattern_bytes[j] == text_bytes[i] {
            i += 1;
            j += 1;
        }

        if j == pattern.len() {
            matches.push(i - j);
            j = lps[j - 1];
        } else if i < text.len() && pattern_bytes[j] != text_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    matches
}

fn compute_lps(pattern: &str, lps: &mut [usize]) {
    let mut length = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern.as_bytes()[i] == pattern.as_bytes()[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
}
