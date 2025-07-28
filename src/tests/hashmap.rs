use crate::*;

#[test]
fn new_hashmap_is_empty() {
    let map: hashmap::HashMap<String, String> = hashmap::HashMap::new();
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
    assert!(map.is_empty());
}

#[test]
fn insert_inserts_one_element() {
    let mut map: hashmap::HashMap<String, String> = hashmap::HashMap::new();
    map.insert(String::from("hello"), String::from("world"));
    assert_eq!(map.len(), 1);
    assert!(map.capacity() > 1);
    assert!(!map.is_empty());
}

#[test]
fn insert_inserts_korrect_pair() {
    let mut map: hashmap::HashMap<&str, &str> = hashmap::HashMap::new();
    map.insert("hello", "world");
    assert_eq!(map.get("hello").unwrap(), &"world");
}

#[test]
fn hashmap_empty_after_clear() {
    let mut map: hashmap::HashMap<&str, &str> = hashmap::HashMap::new();
    map.insert("hello", "world");
    map.clear();
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

#[test]
fn hashmap_with_declared_capacity() {
    let map: hashmap::HashMap<&str, &str> = hashmap::HashMap::with_capacity(10);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10);
    assert!(map.is_empty());
}

#[test]
fn from_creates_correct_values() {
    let map: HashMap<&str, i32> = HashMap::from([
        ("john", 1),
        ("pete", 2),
        ("hans", 3)
    ]);
    assert_eq!(map.get("john").unwrap(), &1);
    assert_eq!(map.get("pete").unwrap(), &2);
    assert_eq!(map.get("hans").unwrap(), &3);
}

#[test]
fn contains_key_returns_false_if_key_does_not_exist() {
    let map: HashMap<&str, i32> = HashMap::from([
        ("john", 1),
        ("pete", 2),
        ("hans", 3)
    ]);
    assert!(!map.contains_key("cage"));
    assert!(!map.contains_key("smith"));
    assert!(!map.contains_key("paul"));
}

#[test]
fn contains_key_returns_true_if_key_exists() {
    let map: HashMap<&str, i32> = HashMap::from([
        ("john", 1),
        ("pete", 2),
        ("hans", 3)
    ]);
    assert!(map.contains_key("john"));
    assert!(map.contains_key("pete"));
    assert!(map.contains_key("hans"));
}

#[test]
fn remove() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    assert_eq!(map.remove(&"hello"), Some("world"));
    assert_eq!(map.remove(&"hello"), None);
}
