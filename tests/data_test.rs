use competitive_programming_lib::DataStructures::binary_heap::MaxHeap;
use competitive_programming_lib::DataStructures::fenwick_tree::FenwickTree;
use competitive_programming_lib::DataStructures::linked_list::LinkedList;
use competitive_programming_lib::DataStructures::trie::Trie;
use competitive_programming_lib::DataStructures::union_find::UnionFind;

#[test]
fn test_fenwick_tree() {
    let mut ft = FenwickTree::new(5);
    ft.update(1, 1);
    ft.update(2, 2);
    ft.update(3, 3);
    assert_eq!(ft.query(3), 6);
    assert_eq!(ft.range_query(2, 3), 5);
}

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);
    uf.union(1, 2);
    uf.union(3, 4);
    assert_eq!(uf.find(1), uf.find(2));
    assert_ne!(uf.find(1), uf.find(3));
}

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("hello");
    assert!(trie.search("hello"));
    assert!(!trie.search("world"));
    assert!(trie.starts_with("hel"));
    assert!(!trie.starts_with("wo"));
}

#[test]
fn test_heap() {
    let mut heap = MaxHeap::new();
    heap.push(3);
    heap.push(5);
    heap.push(1);
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_linked_list() {
    let mut list = LinkedList::new();
    assert!(list.is_empty());
    list.push_front(10);
    list.push_front(20);
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.pop_front(), None);
}
