#[test]
fn tuple_index_access_returns_each_field() {
    let person: (&str, &str, i8) = ("ashu", "pandey", 13);
    assert_eq!(person.0, "ashu");
    assert_eq!(person.1, "pandey");
    assert_eq!(person.2, 13);
}

#[test]
fn tuple_destructuring_binds_each_element() {
    let person = ("ashu", "pandey", 13i8);
    let (first, last, age) = person;
    assert_eq!(first, "ashu");
    assert_eq!(last, "pandey");
    assert_eq!(age, 13);
}

#[test]
fn underscore_ignores_unwanted_fields() {
    let person = ("ashu", "pandey", 13i8);
    let (first, _, _) = person;
    assert_eq!(first, "ashu");
}

#[test]
fn unit_tuple_has_a_single_value() {
    let unit: () = ();
    assert_eq!(unit, ());
}
