pub struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    pub fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let root = self.data.swap_remove(0);
        self.heapify_down(0);
        Some(root)
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] > self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut largest = index;

            if left < len && self.data[left] > self.data[largest] {
                largest = left;
            }

            if right < len && self.data[right] > self.data[largest] {
                largest = right;
            }

            if largest == index {
                break;
            }

            self.data.swap(index, largest);
            index = largest;
        }
    }
}
