pub struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    pub fn new(size: usize) -> Self {
        let mut tree = vec![0; 2 * size];
        SegmentTree { size, tree }
    }

    pub fn build(&mut self, data: &[i32]) {
        for (i, &val) in data.iter().enumerate() {
            self.tree[self.size + i] = val;
        }

        for i in (1..self.size).rev() {
            self.tree[i] = self.tree[i * 2] + self.tree[i * 2 + 1];
        }
    }

    pub fn query(&self, left: usize, right: usize) -> i32 {
        let mut sum = 0;
        let mut left = left + self.size;
        let mut right = right + self.size;

        while left <= right {
            if left % 2 == 1 {
                sum += self.tree[left];
                left += 1;
            }
            if right % 2 == 0 {
                sum += self.tree[right];
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }
        sum
    }
}
