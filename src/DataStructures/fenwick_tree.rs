pub struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
        }
    }

    pub fn update(&mut self, index: usize, value: i32) {
        let mut index = index as isize;
        while index < self.tree.len() as isize {
            self.tree[index as usize] += value;
            index += index & -index;
        }
    }

    pub fn query(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut index = index as isize;
        while index > 0 {
            sum += self.tree[index as usize];
            index -= index & -index;
        }
        sum
    }

    pub fn range_query(&self, left: usize, right: usize) -> i32 {
        self.query(right) - self.query(left - 1)
    }
}
