#[test]
fn str_literal_and_string_are_distinct_types_but_compare_equal() {
    let literal: &str = "hello";
    let owned: String = String::from("hello");
    assert_eq!(literal, owned);
    assert_eq!(owned.as_str(), literal);
}

#[test]
fn push_appends_one_char_and_push_str_appends_a_slice() {
    let mut s = String::from("Hello");
    s.push('a');
    s.push_str("pandey");
    assert_eq!(s, "Helloapandey");
}

#[test]
fn capacity_is_at_least_byte_length_and_grows_as_needed() {
    let mut s = String::with_capacity(4);
    assert!(s.capacity() >= 4);
    s.push_str("a longer string than four bytes");
    assert!(s.capacity() >= s.len());
    assert_eq!(s.len(), "a longer string than four bytes".len());
}

#[test]
fn split_by_whitespace_yields_each_word() {
    let s = String::from("one two three");
    let words: Vec<&str> = s.split(' ').collect();
    assert_eq!(words, vec!["one", "two", "three"]);
}
