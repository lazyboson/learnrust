fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn sum(xs: &[i32]) -> i32 {
    let mut total = 0;
    for &x in xs {
        total += x;
    }
    total
}

#[test]
fn string_slice_extracts_a_byte_range() {
    let s = String::from("hello world");
    assert_eq!(&s[0..5], "hello");
    assert_eq!(&s[6..11], "world");
}

#[test]
fn slice_shorthands_are_equivalent_to_explicit_ranges() {
    let s = String::from("hello world");
    assert_eq!(&s[..5], &s[0..5]);
    assert_eq!(&s[6..], &s[6..s.len()]);
    assert_eq!(&s[..], s.as_str());
}

#[test]
fn first_word_returns_text_before_first_space() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("oneword"), "oneword"); // no space — whole string
    assert_eq!(first_word(""), "");                // empty
}

#[test]
fn first_word_accepts_both_string_slice_and_string_literal() {
    let owned = String::from("foo bar");
    // &String coerces to &str at the call site (deref coercion)
    assert_eq!(first_word(&owned), "foo");
    // string literal IS a &str directly
    assert_eq!(first_word("baz qux"), "baz");
}

#[test]
fn array_slice_borrows_a_view_into_the_array() {
    let arr = [1, 2, 3, 4, 5];
    let middle: &[i32] = &arr[1..4];
    assert_eq!(middle, &[2, 3, 4]);
    assert_eq!(middle.len(), 3);
    assert_eq!(sum(middle), 9);
}

#[test]
fn vec_slice_works_the_same_as_array_slice() {
    let v = vec![10, 20, 30, 40, 50];
    let slice: &[i32] = &v[1..4];
    assert_eq!(slice, &[20, 30, 40]);
    assert_eq!(sum(slice), 90);
}

#[test]
fn string_literal_is_a_static_str_slice() {
    let literal: &str = "static";
    assert_eq!(literal.len(), 6);
    // string literals live for the whole program (&'static str)
    let _check: &'static str = literal;
}
