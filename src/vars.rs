pub fn var() {
    let name = "Ashutosh";
    let mut age = 29;

    println!("my name is {} and I am {}", name, age);
     age = 45;
    println!("my name is {} and I am {}", name, age);

    //define a const
    const ID:i32 = 001;
    println!("ID: {}", ID);

    //assign multiple variable
    let (my_name, my_age) = ("ashutosh", 35);
    println!("my name {} age{}", my_name, my_age);
}