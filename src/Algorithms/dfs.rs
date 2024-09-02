pub fn dfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>) {
    visited[start] = true;
    for &neighbor in &graph[start] {
        if !visited[neighbor] {
            dfs(graph, neighbor, visited)
        }
    }
}
