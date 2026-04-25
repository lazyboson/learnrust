use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[test]
fn hashmap_inserts_and_reads_back_values() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("missing"), None);
}

#[test]
fn hashmap_insert_overwrites_existing_key() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("k", 1);
    map.insert("k", 99);
    assert_eq!(map.get("k"), Some(&99));
    assert_eq!(map.len(), 1);
}

#[test]
fn hashset_deduplicates_inserts() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2);
    assert_eq!(set.len(), 2);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
}

#[test]
fn btreemap_iterates_in_key_order() {
    let mut bmap: BTreeMap<&str, i32> = BTreeMap::new();
    bmap.insert("two", 2);
    bmap.insert("one", 1);
    bmap.insert("three", 3);
    let keys: Vec<&&str> = bmap.keys().collect();
    assert_eq!(keys, vec![&"one", &"three", &"two"]);
}

#[test]
fn btreeset_iterates_sorted_and_deduplicates() {
    let mut bset: BTreeSet<i32> = BTreeSet::new();
    bset.insert(3);
    bset.insert(1);
    bset.insert(2);
    bset.insert(1);
    let values: Vec<i32> = bset.iter().copied().collect();
    assert_eq!(values, vec![1, 2, 3]);
}
