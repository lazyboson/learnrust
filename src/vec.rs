pub fn usevec() {
    //vectors are resizable arrays
    let mut vectors: Vec<i32> = vec![1,2,3,4,5];


    vectors.push(14);
    vectors.push(14);
    vectors.push(34);
    println!("vector is {:?}", vectors);
    for x in vectors.iter() {
        println!("vector val : {}", x);
    }

    let mut vectors2: Vec<i32> = Vec::new();
    for i in 0..10 {
        vectors2.push(i);
    }

    println!("vector2 is {:?}", vectors2);
}