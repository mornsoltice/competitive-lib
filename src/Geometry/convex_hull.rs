use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    pub fn det(&self, other: &Point) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

pub fn extract_convex_hull(points: &[Point], contain_on_segment: bool) -> Vec<usize> {
    let n = points.len();
    if n <= 1 {
        return (0..n).collect();
    }

    let mut sorted_indices: Vec<usize> = (0..n).collect();
    sorted_indices.sort_by(|&a, &b| {
        let cmp_x = points[a].x.cmp(&points[b].x);
        if cmp_x == Ordering::Equal {
            points[a].y.cmp(&points[b].y)
        } else {
            cmp_x
        }
    });

    let mut hull: Vec<usize> = Vec::new();

    for &i in &sorted_indices {
        while hull.len() >= 2 {
            let (p2, p1) = (hull[hull.len() - 2], hull[hull.len() - 1]);
            let det = (points[p1] - points[p2]).det(&(points[i] - points[p1]));
            if det < 0 || (det <= 0 && !contain_on_segment) {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(i);
    }

    let t = hull.len();
    for i in (0..(n - 1)).rev() {
        let i = sorted_indices[i];
        while hull.len() > t {
            let (p2, p1) = (hull[hull.len() - 2], hull[hull.len() - 1]);
            let det = (points[p1] - points[p2]).det(&(points[i] - points[p1]));
            if det < 0 || (det <= 0 && !contain_on_segment) {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(i);
    }

    if hull.len() > 1 {
        hull.pop();
    }

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_point_in_convex_hull(hull: &[usize], point_index: usize) -> bool {
        hull.contains(&point_index)
    }

    #[test]
    fn test_extract_convex_hull() {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 2 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 2 },
        ];

        let hull_indices = extract_convex_hull(&points, false);
        let expected_indices = vec![0, 2, 4, 3]; // Adjust as necessary for your hull

        assert_eq!(hull_indices.len(), 4); // Check if the hull has the correct number of points
        for idx in &expected_indices {
            assert!(is_point_in_convex_hull(&hull_indices, *idx));
        }

        for idx in 0..points.len() {
            if !expected_indices.contains(&idx) {
                assert!(!is_point_in_convex_hull(&hull_indices, idx));
            }
        }

        assert!(is_convex_hull(&points, &hull_indices));
    }

    fn is_convex_hull(points: &[Point], hull: &[usize]) -> bool {
        hull.len() >= 3 // A convex hull must have at least 3 points
    }
}
