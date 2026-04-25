#[test]
fn array_literal_initializes_each_element() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);
    assert_eq!(arr[0], 1);
    assert_eq!(arr[4], 5);
}

#[test]
fn mut_array_allows_index_assignment() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[2] = 20;
    assert_eq!(arr[2], 20);
}

#[test]
fn repeat_syntax_creates_n_copies_of_one_value() {
    let zeros: [i32; 5] = [0; 5];
    assert_eq!(zeros, [0, 0, 0, 0, 0]);
    let threes = [3; 5];
    assert_eq!(threes, [3, 3, 3, 3, 3]);
}

#[test]
fn slice_borrows_a_view_into_the_array() {
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    assert_eq!(slice.len(), 3);
}

#[test]
fn get_returns_none_for_out_of_bounds_index() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(arr.get(99), None);
    assert_eq!(arr.get(0), Some(&1));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn indexing_out_of_bounds_panics_at_runtime() {
    let arr = [1, 2, 3, 4, 5];
    // computed at runtime so the compiler doesn't const-fold the panic
    let i: usize = std::hint::black_box(99);
    let _ = arr[i];
}
