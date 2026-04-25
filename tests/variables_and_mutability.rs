#[test]
fn mut_allows_reassigning_same_type() {
    let mut age = 29;
    assert_eq!(age, 29);
    age = 45;
    assert_eq!(age, 45);
}

#[test]
fn const_value_is_compile_time_constant() {
    const ID: i32 = 1;
    assert_eq!(ID, 1);
}

#[test]
fn tuple_destructure_binds_each_element() {
    let (name, age) = ("ashutosh", 35);
    assert_eq!(name, "ashutosh");
    assert_eq!(age, 35);
}

#[test]
fn shadowing_creates_a_new_binding() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    assert_eq!(x, 12);
}

#[test]
fn shadowing_can_change_type() {
    let spaces = "   ";
    let spaces = spaces.len();
    // the binding is now usize, not &str
    let _check: usize = spaces;
    assert_eq!(spaces, 3);
}

#[test]
fn shadowing_inside_block_does_not_escape() {
    let z = 5;
    {
        let z = z * 10;
        assert_eq!(z, 50);
    }
    assert_eq!(z, 5);
}
