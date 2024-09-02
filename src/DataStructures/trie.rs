#[derive(Default)]
pub struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node
                .children
                .entry(c)
                .or_insert_with(TrieNode::default);
        }
        current_node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }
        current_node.is_end_of_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current_node = &self.root;
        for c in prefix.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }
        true
    }
}
