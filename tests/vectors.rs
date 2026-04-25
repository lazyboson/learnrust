#[test]
fn vec_macro_creates_vec_with_initial_elements() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert_eq!(v.len(), 5);
    assert_eq!(v[0], 1);
    assert_eq!(v[4], 5);
}

#[test]
fn push_appends_to_end_and_keeps_duplicates() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    v.push(14);
    v.push(14);
    v.push(34);
    assert_eq!(v, vec![1, 2, 3, 4, 5, 14, 14, 34]);
}

#[test]
fn iter_yields_references_in_order() {
    let v = vec![10, 20, 30];
    let collected: Vec<i32> = v.iter().copied().collect();
    assert_eq!(collected, vec![10, 20, 30]);
}

#[test]
fn empty_vec_built_via_range_push_matches_collected_range() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..10 {
        v.push(i);
    }
    assert_eq!(v, (0..10).collect::<Vec<i32>>());
}
