#[test]
fn if_is_an_expression_both_arms_same_type() {
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    assert_eq!(label, "odd");

    let m = 8;
    let label2 = if m % 2 == 0 { "even" } else { "odd" };
    assert_eq!(label2, "even");
}

#[test]
fn loop_break_can_return_a_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

#[test]
fn labeled_break_exits_outer_loop() {
    let mut count = 0;
    'outer: loop {
        loop {
            count += 1;
            if count == 5 {
                break 'outer;
            }
        }
    }
    assert_eq!(count, 5);
}

#[test]
fn while_runs_until_condition_is_false() {
    let mut k = 3;
    let mut iterations = 0;
    while k != 0 {
        iterations += 1;
        k -= 1;
    }
    assert_eq!(iterations, 3);
    assert_eq!(k, 0);
}

#[test]
fn for_iterates_over_each_element() {
    let arr = [10, 20, 30];
    let mut total = 0;
    for v in arr.iter() {
        total += v;
    }
    assert_eq!(total, 60);
}

#[test]
fn range_is_exclusive_of_end() {
    let collected: Vec<i32> = (1..4).collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn range_inclusive_includes_end() {
    let collected: Vec<i32> = (1..=4).collect();
    assert_eq!(collected, vec![1, 2, 3, 4]);
}

#[test]
fn rev_yields_iterator_in_reverse() {
    let collected: Vec<i32> = (1..4).rev().collect();
    assert_eq!(collected, vec![3, 2, 1]);
}
