fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn append_world_owned(mut s: String) -> String {
    s.push_str(", world");
    s
}

#[test]
fn mut_ref_modifies_callers_string_in_place() {
    let mut s = String::from("hello");
    append_world(&mut s);
    // same binding, same allocation — only the contents grew
    assert_eq!(s, "hello, world");
}

#[test]
fn mut_ref_can_be_called_repeatedly() {
    // each call's borrow ends when the call returns (NLL), so we can
    // borrow mutably again on the next line.
    let mut s = String::from("hi");
    append_world(&mut s);
    append_world(&mut s);
    assert_eq!(s, "hi, world, world");
}

#[test]
fn caller_keeps_ownership_after_mut_ref_call() {
    // we can keep using `s` after the function returns because we never
    // moved it — we only lent a &mut borrow.
    let mut s = String::from("hello");
    append_world(&mut s);
    assert_eq!(s.len(), 12);   // "hello, world"
    s.push('!');
    assert_eq!(s, "hello, world!");
}

#[test]
fn move_and_return_round_trips_the_value() {
    let s = String::from("hello");
    let s = append_world_owned(s);          // move IN, return OUT
    assert_eq!(s, "hello, world");
    // post-call `s` is a NEW (shadowed) binding to the returned String
}

#[test]
fn move_and_return_caller_does_not_need_mut() {
    // the caller's binding is immutable. the `mut` is on the FUNCTION's
    // parameter list, which is independent of the caller's mutability.
    let s = String::from("hi");
    let s = append_world_owned(s);
    assert_eq!(s, "hi, world");
}

#[test]
fn move_and_return_chains() {
    // builder-style: consecutive moves through the same function
    let s = String::from("a");
    let s = append_world_owned(s);
    let s = append_world_owned(s);
    assert_eq!(s, "a, world, world");
}
