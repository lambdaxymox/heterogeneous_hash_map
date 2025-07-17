#![no_std]
extern crate std;

use heterogeneous_hash_map::{HeterogeneousHashMap, Key};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use core::any;
use std::vec::Vec;
use std::boxed::Box;
use std::string::String;

#[test]
fn test_heterogeneous_hash_map_one_type_zst1() {
    let mut het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), None);
    assert_eq!(het_map.len::<()>(), None);

    het_map.insert_type::<()>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    het_map.insert(Key::new(0), ());

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(false));
    assert_eq!(het_map.len::<()>(), Some(1));
    assert_eq!(het_map.get(&Key::new(0)), Some(&()));
}

#[test]
fn test_heterogeneous_hash_map_one_type_zst2() {
    let mut het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), None);
    assert_eq!(het_map.len::<()>(), None);

    het_map.insert_type::<()>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    het_map.insert(Key::new(0), ());
    het_map.insert(Key::new(1), ());
    het_map.insert(Key::new(2), ());

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(false));
    assert_eq!(het_map.len::<()>(), Some(3));
    assert_eq!(het_map.get(&Key::new(0)), Some(&()));
}

#[test]
fn test_heterogeneous_hash_map_one_type_zst3() {
    let mut het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), None);
    assert_eq!(het_map.len::<()>(), None);

    het_map.insert_type::<()>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    let len = 1204;
    for i in 0..len {
        het_map.insert(Key::new(i), ());
    }

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(false));
    assert_eq!(het_map.len::<()>(), Some(len));

    for i in 0..len {
        assert!(het_map.contains_key::<()>(&Key::new(i)));
    }

    assert!(!het_map.contains_key::<()>(&Key::new(len)));
}

#[test]
fn test_heterogeneous_hash_map_one_type_zst4() {
    let mut het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), None);
    assert_eq!(het_map.len::<()>(), None);

    het_map.insert_type::<()>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    let len = 1024;
    for i in 0..len {
        het_map.insert(Key::new(i), ());
    }

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(false));
    assert_eq!(het_map.len::<()>(), Some(len));

    for i in 0..len {
        assert_eq!(het_map.get::<()>(&Key::new(i)), Some(&()));
    }

    assert_eq!(het_map.get::<()>(&Key::new(len)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_extend1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_extend2() {
    let mut het_map = HeterogeneousHashMap::new();
    let vec = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    het_map.extend::<_, String>(vec);

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.remove::<String>(&Key::new(1_usize)), Some(String::from("foo")));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<String>(&Key::new(1_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.remove::<String>(&Key::new(2_usize)), Some(String::from("bar")));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<String>(&Key::new(2_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove3() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.remove::<String>(&Key::new(3_usize)), Some(String::from("baz")));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<String>(&Key::new(3_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_contains_key() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert!(het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<String>(&Key::new(1_usize)), Some(String::from("foo")));

    assert!(!het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<String>(&Key::new(2_usize)), Some(String::from("bar")));

    assert!(!het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<String>(&Key::new(3_usize)), Some(String::from("baz")));

    assert!(!het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_get() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String>(&Key::new(1_usize)), Some(String::from("foo")));

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String>(&Key::new(2_usize)), Some(String::from("bar")));

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String>(&Key::new(3_usize)), Some(String::from("baz")));

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_get_key_value() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String>(&Key::new(1_usize)), Some(String::from("foo")));

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String>(&Key::new(2_usize)), Some(String::from("bar")));

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String>(&Key::new(3_usize)), Some(String::from("baz")));

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_entry1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<String>(&Key::new(1_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_entry2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<String>(&Key::new(2_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_entry3() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<String>(&Key::new(3_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_entry_contains_key() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert!(het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<String>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    assert!(!het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<String>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    assert!(!het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<String>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    assert!(!het_map.contains_key::<String>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String>(&Key::new(4_usize)));
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_entry_get() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    assert_eq!(het_map.get::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_remove_entry_get_key_value() {
    let mut het_map = HeterogeneousHashMap::new();
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    assert_eq!(het_map.get_key_value::<String>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get_key_value::<String>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_iter1() {
    let entries = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries);

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.iter();

    assert_eq!(iter.next(), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(iter.next(), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(iter.next(), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(iter.next(), Some((&Key::new(4_usize), &String::from("quux"))));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_iter2() {
    let entries = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected = entries.clone();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<(Key<String>, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_one_type_iter3() {
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.iter();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_iter4() {
    let expected = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<(Key<String>, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_one_type_keys1() {
    let entries = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries);

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.keys();

    assert_eq!(iter.next(), Some(&Key::new(1_usize)));
    assert_eq!(iter.next(), Some(&Key::new(2_usize)));
    assert_eq!(iter.next(), Some(&Key::new(3_usize)));
    assert_eq!(iter.next(), Some(&Key::new(4_usize)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_keys2() {
    let entries = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected: Vec<Key<String>> = entries.iter().map(|(k, _v)| k).cloned().collect();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<Key<String>> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_one_type_keys3() {
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.keys();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_keys4() {
    let expected = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<Key<String>> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_one_type_values1() {
    let entries = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries);

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.values();

    assert_eq!(iter.next(), Some(&String::from("foo")));
    assert_eq!(iter.next(), Some(&String::from("bar")));
    assert_eq!(iter.next(), Some(&String::from("baz")));
    assert_eq!(iter.next(), Some(&String::from("quux")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_values2() {
    let entries = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected: Vec<String> = entries.iter().map(|(_k, v)| v).cloned().collect();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<String> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_one_type_values3() {
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.values();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_one_type_values4() {
    let expected: Vec<String> = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<String> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_one_type_clear1() {
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<Box<dyn any::Any>>();

    let map = het_map.get_map_mut::<Box<dyn any::Any>>().unwrap();
    assert!(map.is_empty());

    map.clear();

    assert!(map.is_empty());
}

#[test]
fn test_heterogeneous_hash_map_one_type_clear2() {
    let mut het_map = HeterogeneousHashMap::new();

    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<i32>();

    assert_eq!(het_map.len::<i32>(), Some(0));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(map.is_empty());

        map.extend([(Key::new(1_usize), i32::MIN), (Key::new(2_usize), 0_i32), (Key::new(3_usize), i32::MAX)]);
    }
    assert_eq!(het_map.len::<i32>(), Some(3));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(!map.is_empty());

        map.clear();
    }
    assert_eq!(het_map.len::<i32>(), Some(0));
}
