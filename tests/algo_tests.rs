use competitive_programming_lib::Algorithms::bfs::bfs;
use competitive_programming_lib::Algorithms::dfs::dfs;
use competitive_programming_lib::Algorithms::gcd::gcd;
use competitive_programming_lib::Algorithms::kmp::kmp_pattern_search;
use competitive_programming_lib::Algorithms::prime_factors::prime_factors;

#[test]
fn test_bfs() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    let order = bfs(&graph, 0);
    assert_eq!(order, vec![0, 1, 2, 3]);
}

#[test]
fn test_dfs() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    let mut visited = vec![false; 4];
    dfs(&graph, 0, &mut visited);
    assert_eq!(visited, vec![true, true, true, true]);
}

#[test]
fn test_kmp() {
    assert_eq!(kmp_pattern_search("abxabcabcaby", "abcaby"), vec![6]);
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(28), vec![2, 2, 7]);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
}
