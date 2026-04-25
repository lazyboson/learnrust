#[test]
fn integer_literals_in_different_bases_have_equal_values() {
    assert_eq!(0xff, 255);
    assert_eq!(0o77, 63);
    assert_eq!(0b1111_0000, 240);
    assert_eq!(b'A', 65u8);
}

#[test]
fn underscores_in_numeric_literals_are_ignored() {
    assert_eq!(1_000_000, 1000000);
    assert_eq!(0b1111_0000, 0b11110000);
}

#[test]
fn type_suffixed_literal_has_that_type() {
    let typed = 57u32;
    let _check: u32 = typed;
    assert_eq!(typed, 57);
}

#[test]
fn wrapping_add_wraps_on_overflow() {
    let a: u8 = 250;
    assert_eq!(a.wrapping_add(10), 4); // 260 mod 256 = 4
}

#[test]
fn checked_add_returns_none_on_overflow() {
    let a: u8 = 250;
    assert_eq!(a.checked_add(10), None);
    assert_eq!(a.checked_add(5), Some(255));
}

#[test]
fn integer_division_truncates_toward_zero() {
    assert_eq!(7i32 / 3, 2);
    assert_eq!(-7i32 / 3, -2);
}

#[test]
fn char_is_four_bytes_unicode_scalar() {
    assert_eq!(std::mem::size_of::<char>(), 4);
    let kanji = '日';
    let emoji = '😻';
    // both are valid char values regardless of byte length in UTF-8
    assert_eq!(kanji.len_utf8(), 3);
    assert_eq!(emoji.len_utf8(), 4);
}

#[test]
fn unit_type_is_inhabited_by_a_single_value() {
    let a: () = ();
    let b: () = ();
    assert_eq!(a, b);
}
