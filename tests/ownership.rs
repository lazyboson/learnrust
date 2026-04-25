fn takes_and_gives_back(s: String) -> String {
    s
}

fn gives_ownership() -> String {
    String::from("from function")
}

fn append_bang(mut s: String) -> String {
    s.push('!');
    s
}

#[test]
fn clone_creates_an_independent_copy() {
    let a = String::from("hi");
    let b = a.clone();
    // both bindings remain valid because clone allocates new heap memory
    assert_eq!(a, "hi");
    assert_eq!(b, "hi");
    assert_eq!(a, b);
    // their underlying allocations are distinct (different ptrs)
    assert_ne!(a.as_ptr(), b.as_ptr());
}

#[test]
fn copy_types_remain_usable_after_assignment() {
    let x = 5i32;
    let y = x;
    assert_eq!(x, 5);
    assert_eq!(y, 5);

    // tuples of Copy types are also Copy
    let p = (1i32, true, 'a');
    let q = p;
    assert_eq!(p, (1, true, 'a'));
    assert_eq!(q, (1, true, 'a'));
}

#[test]
fn function_can_return_ownership_of_a_new_value() {
    let s = gives_ownership();
    assert_eq!(s, "from function");
}

#[test]
fn ownership_round_trips_through_function() {
    let s = String::from("hello");
    let s = takes_and_gives_back(s);    // moved in, moved back out
    assert_eq!(s, "hello");
}

#[test]
fn function_can_mutate_owned_argument_then_return_it() {
    // `mut s` in the parameter list makes the OWNED parameter mutable inside
    // the function. it does NOT mean the caller passes a mutable reference.
    let s = append_bang(String::from("hi"));
    assert_eq!(s, "hi!");
}
