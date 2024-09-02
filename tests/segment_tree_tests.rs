use competitive_lib::DataStructures::segment_tree;

#[test]
fn test_segment_tree() {
    let data = vec![1, 2, 3, 4, 5];
    let mut st = segment_tree::SegmentTree::new(data.len());
    st.build(&data);
    assert_eq!(st.query(1, 3), 9);
}
