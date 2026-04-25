fn main() {
    let mut members: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", members);

    members[2] = 20;
    //get single valu
    println!("first val: {} and third is {} ", members[0], members[2]);

    //slices from array
    let slice: &[i32] = &members;

    println!("Slice is {:?}", slice);

    // --- repeat syntax: [value; count] ---
    // creates an array of `count` copies of `value`. handy for zero-initing.
    let zeros: [i32; 5] = [0; 5];        // [0, 0, 0, 0, 0]
    let threes = [3; 5];                  // type inferred as [i32; 5]
    println!("zeros = {:?}, threes = {:?}", zeros, threes);

    // --- bounds checking is at runtime ---
    // unlike C, indexing past the end PANICS — Rust never lets you read
    // garbage memory. uncommenting the next line would panic at runtime:
    //   let bad = members[99];
    //   thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 99'

    // safe alternative: .get(i) returns Option<&T> instead of panicking
    let safe = members.get(99);
    println!("members.get(99) = {:?}", safe);   // None
    let ok = members.get(0);
    println!("members.get(0) = {:?}", ok);      // Some(1)
}