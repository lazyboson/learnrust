pub fn usearray() {
    let mut members: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", members);

    members[2] = 20;
    //get single valu
    println!("first val: {} and third is {} ", members[0], members[2]);

    //slices from array
    let slice: &[i32] = &members;

    println!("Slice is {:?}", slice)

}