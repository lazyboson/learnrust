pub fn datatype() {
    //primitive types
    let x = 32;
    let y = 2.5;

    let yy: i64 = 345;

    //find max size
    println!("max i32 {}", std::i32::MAX);

    //boolean
    let is_active = true;
    println!("{} {} {} is active ? {}", x, y, yy, is_active);

    let a1 = '1';
    println!("this is character {}", a1);
}