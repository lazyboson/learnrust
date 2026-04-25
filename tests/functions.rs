fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn double(x: i32) -> i32 {
    x * 2
}

fn returns_five() -> i32 {
    return 5;
}

#[test]
fn add_returns_sum_via_implicit_return() {
    assert_eq!(add(3, 4), 7);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn double_returns_x_times_two() {
    assert_eq!(double(3), 6);
    assert_eq!(double(0), 0);
}

#[test]
fn explicit_return_works_too() {
    assert_eq!(returns_five(), 5);
}

#[test]
fn block_expression_evaluates_to_final_expression() {
    let value = {
        let a = 2;
        let b = 3;
        a + b
    };
    assert_eq!(value, 5);
}

#[test]
fn block_with_only_statements_evaluates_to_unit() {
    // every line ends in `;` so the block has no trailing expression -> ()
    let value: () = {
        let _x = 5;
        let _y = 6;
    };
    assert_eq!(value, ());
}
