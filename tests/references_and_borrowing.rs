fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

#[test]
fn immutable_reference_does_not_take_ownership() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    // s is still usable here BECAUSE we passed &s, not s
    assert_eq!(len, 5);
    assert_eq!(s, "hello");
}

#[test]
fn mutable_reference_can_modify_through_it() {
    let mut s = String::from("hello");
    append_world(&mut s);
    assert_eq!(s, "hello, world");
}

#[test]
fn many_immutable_references_can_coexist() {
    let s = String::from("shared");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    // all three are valid simultaneously — no &mut is involved
    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r1.len(), 6);
}

#[test]
fn mut_ref_after_imm_refs_finish_using_is_ok() {
    // non-lexical lifetimes (NLL): borrows END at last use, not at end of scope.
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    assert_eq!(r1.len(), 5);
    assert_eq!(r2.len(), 5);
    // r1, r2 not used after this line — their borrows end here.

    let r3 = &mut s;
    r3.push_str(", world");
    // r3 not used after this — its borrow ends here.

    // s is now usable again because no live borrow exists
    assert_eq!(s, "hello, world");
}

#[test]
fn borrow_ends_after_last_use_allowing_mutation() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    assert_eq!(*first, 1);
    // first not used after this — borrow ends.

    v.push(4);
    assert_eq!(v, vec![1, 2, 3, 4]);
}
