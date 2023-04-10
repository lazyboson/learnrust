pub fn stl() {
    let mut map: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }    

    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2);

    for x in set.iter() {
        println!("set val : {}", x);
    }

    //BTreeMap and BTreeSet are sorted
    let mut bmap: std::collections::BTreeMap<&str, i32> = std::collections::BTreeMap::new();
    bmap.insert("one", 1);
    bmap.insert("two", 2);
    bmap.insert("three", 3);
    for (key, value) in bmap.iter() {
        println!("{}: {}", key, value);
    }

    let mut bset: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();
    bset.insert(1);
    bset.insert(2);

    for x in bset.iter() {
        println!("bset val : {}", x);
    }
}