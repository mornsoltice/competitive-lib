pub struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, value: i32) {
        let new_node = Box::new(ListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
