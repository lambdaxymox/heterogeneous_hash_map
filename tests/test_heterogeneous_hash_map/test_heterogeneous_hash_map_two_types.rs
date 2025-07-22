use heterogeneous_hash_map::{HeterogeneousHashMap, Key};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use core::any;
use std::vec::Vec;
use std::boxed::Box;
use std::string::{
    String,
    ToString,
};

#[test]
fn test_heterogeneous_hash_map_two_types_zst1() {
    let mut het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), None);
    assert_eq!(het_map.len::<()>(), None);

    assert!(!het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), None);
    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<()>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    assert!(!het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), None);
    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<i32>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    assert!(het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), Some(true));
    assert_eq!(het_map.len::<i32>(), Some(0));

    het_map.insert(Key::new(0_i32), ());

    het_map.insert(Key::new(0_i32), i32::MIN);
    het_map.insert(Key::new(1_i32), i32::MAX);

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(false));
    assert_eq!(het_map.len::<()>(), Some(1));
    assert_eq!(het_map.get::<(), _>(&Key::new(0_i32)), Some(&()));

    assert!(het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), Some(false));
    assert_eq!(het_map.len::<i32>(), Some(2));
    assert_eq!(het_map.get::<i32, _>(&Key::new(0_i32)), Some(&i32::MIN));
    assert_eq!(het_map.get::<i32, _>(&Key::new(1_i32)), Some(&i32::MAX));
}

#[test]
fn test_heterogeneous_hash_map_two_types_zst2() {
    let mut het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), None);
    assert_eq!(het_map.len::<()>(), None);

    assert!(!het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), None);
    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<()>();

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(true));
    assert_eq!(het_map.len::<()>(), Some(0));

    assert!(!het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), None);
    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert(Key::new(0_i32), ());
    het_map.insert(Key::new(1_i32), ());
    het_map.insert(Key::new(2_i32), ());

    het_map.insert(Key::new(0_i32), i32::MIN);
    het_map.insert(Key::new(1_i32), i32::MAX);

    assert!(het_map.contains_type::<()>());
    assert_eq!(het_map.is_empty::<()>(), Some(false));
    assert_eq!(het_map.len::<()>(), Some(3));
    assert_eq!(het_map.get::<(), _>(&Key::new(0_i32)), Some(&()));

    assert!(het_map.contains_type::<i32>());
    assert_eq!(het_map.is_empty::<i32>(), Some(false));
    assert_eq!(het_map.len::<i32>(), Some(2));
    assert_eq!(het_map.get::<i32, _>(&Key::new(0_i32)), Some(&i32::MIN));
    assert_eq!(het_map.get::<i32, _>(&Key::new(1_i32)), Some(&i32::MAX));
}

#[test]
fn test_heterogeneous_hash_map_two_types_extend1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    het_map.extend::<_, i32>(array1);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_extend2() {
    let mut het_map = HeterogeneousHashMap::new();
    let vec1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let vec2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    het_map.extend::<_, i32>(vec1);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    het_map.extend::<_, String>(vec2);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<String, _>(&Key::new(1_usize)), Some(String::from("foo")));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<String, _>(&Key::new(1_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<String, _>(&Key::new(2_usize)), Some(String::from("bar")));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<String, _>(&Key::new(2_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove3() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<String, _>(&Key::new(3_usize)), Some(String::from("baz")));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<String, _>(&Key::new(3_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove4() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(1_usize)), Some(20_i32));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<i32, _>(&Key::new(1_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove5() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(2_usize)), Some(30_i32));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<i32, _>(&Key::new(2_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove6() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(3_usize)), Some(40_i32));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<i32, _>(&Key::new(3_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove7() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(4_usize)), Some(50_i32));

    for _ in 0..100 {
        assert_eq!(het_map.remove::<i32, _>(&Key::new(4_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_contains_key1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<i32, _>(&Key::new(1_usize)), Some(20_i32));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<i32, _>(&Key::new(2_usize)), Some(30_i32));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<i32, _>(&Key::new(3_usize)), Some(40_i32));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<i32, _>(&Key::new(4_usize)), Some(50_i32));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_contains_key2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<String, _>(&Key::new(1_usize)), Some(String::from("foo")));

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(!het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<String, _>(&Key::new(2_usize)), Some(String::from("bar")));

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(!het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove::<String, _>(&Key::new(3_usize)), Some(String::from("baz")));

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(!het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_get1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(1_usize)), Some(20_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(2_usize)), Some(30_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(3_usize)), Some(40_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(4_usize)), Some(50_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_get2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String, _>(&Key::new(1_usize)), Some(String::from("foo")));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String, _>(&Key::new(2_usize)), Some(String::from("bar")));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String, _>(&Key::new(3_usize)), Some(String::from("baz")));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_get_key_value1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(1_usize)), Some(20_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(2_usize)), Some(30_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(3_usize)), Some(40_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<i32, _>(&Key::new(4_usize)), Some(50_i32));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_get_key_value2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String, _>(&Key::new(1_usize)), Some(String::from("foo")));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String, _>(&Key::new(2_usize)), Some(String::from("bar")));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove::<String, _>(&Key::new(3_usize)), Some(String::from("baz")));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<String, _>(&Key::new(1_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<String, _>(&Key::new(2_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry3() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<String, _>(&Key::new(3_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry4() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1_usize)), Some((Key::new(1_usize), 20_i32)));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry5() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(2_usize)), Some((Key::new(2_usize),30_i32)));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(2_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry6() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(3_usize)), Some((Key::new(3_usize), 40_i32)));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(3_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry7() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(4_usize)), Some((Key::new(4_usize), 50_i32)));

    for _ in 0..100 {
        assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(4_usize)), None);
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry_contains_key1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array);

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1_usize)), Some((Key::new(1_usize), 20_i32)));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(2_usize)), Some((Key::new(2_usize), 30_i32)));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(3_usize)), Some((Key::new(3_usize), 40_i32)));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(4_usize)), Some((Key::new(4_usize), 50_i32)));

    assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry_contains_key2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array);

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(!het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(!het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(3_usize)));
    assert!(het_map.contains_key::<i32, _>(&Key::new(4_usize)));
    assert!(!het_map.contains_key::<i32, _>(&Key::new(5_usize)));

    assert!(!het_map.contains_key::<String, _>(&Key::new(1_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(2_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(3_usize)));
    assert!(!het_map.contains_key::<String, _>(&Key::new(4_usize)));
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry_get1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1_usize)), Some((Key::new(1_usize), 20_i32)));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(2_usize)), Some((Key::new(2_usize), 30_i32)));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(3_usize)), Some((Key::new(3_usize), 40_i32)));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(4_usize)), Some((Key::new(4_usize), 50_i32)));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), None);
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry_get2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), Some(&String::from("foo")));
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), Some(&String::from("bar")));
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("baz")));
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&30_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&40_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(4_usize)), Some(&50_i32));
    assert_eq!(het_map.get::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry_get_key_value1() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &20_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1_usize)), Some((Key::new(1_usize), 20_i32)));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(2_usize)), Some((Key::new(2_usize), 30_i32)));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(3_usize)), Some((Key::new(3_usize), 40_i32)));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(4_usize)), Some((Key::new(4_usize), 50_i32)));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), None);
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_remove_entry_get_key_value2() {
    let mut het_map = HeterogeneousHashMap::new();
    let array1 = [
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let array2 = [
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
    ];
    het_map.extend::<_, i32>(array1);
    het_map.extend::<_, String>(array2);

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &20_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(1_usize)), Some((Key::new(1_usize), String::from("foo"))));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &20_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(2_usize)), Some((Key::new(2_usize), String::from("bar"))));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &20_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);

    assert_eq!(het_map.remove_entry::<String, _>(&Key::new(3_usize)), Some((Key::new(3_usize), String::from("baz"))));

    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), Some((&Key::new(1_usize), &20_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(2_usize)), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(3_usize)), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(4_usize)), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(5_usize)), None);

    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(2_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(3_usize)), None);
    assert_eq!(het_map.get_key_value::<String, _>(&Key::new(4_usize)), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter1() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<i32>().unwrap();
    let mut iter = map.iter();

    assert_eq!(iter.next(), Some((&Key::new(1_usize), &20_i32)));
    assert_eq!(iter.next(), Some((&Key::new(2_usize), &30_i32)));
    assert_eq!(iter.next(), Some((&Key::new(3_usize), &40_i32)));
    assert_eq!(iter.next(), Some((&Key::new(4_usize), &50_i32)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter2() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.iter();

    assert_eq!(iter.next(), Some((&Key::new(1_usize), &String::from("foo"))));
    assert_eq!(iter.next(), Some((&Key::new(2_usize), &String::from("bar"))));
    assert_eq!(iter.next(), Some((&Key::new(3_usize), &String::from("baz"))));
    assert_eq!(iter.next(), Some((&Key::new(4_usize), &String::from("quux"))));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter3() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected = entries1.clone();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<i32>().unwrap();
    let result: Vec<(Key<usize, i32>, i32)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter4() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected = entries2.clone();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<(Key<usize, String>, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter5() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<i32>().unwrap();
    let mut iter = map.iter();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter6() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.iter();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter7() {
    let expected = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<i32>().unwrap();
    let result: Vec<(Key<usize, i32>, i32)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_iter8() {
    let expected = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<(Key<usize, String>, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys1() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.keys();

    assert_eq!(iter.next(), Some(&Key::new(1_usize)));
    assert_eq!(iter.next(), Some(&Key::new(2_usize)));
    assert_eq!(iter.next(), Some(&Key::new(3_usize)));
    assert_eq!(iter.next(), Some(&Key::new(4_usize)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys3() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected: Vec<Key<usize, String>> = entries2.iter().map(|(k, _v)| k).cloned().collect();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<Key<usize, String>> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys4() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected: Vec<Key<usize, String>> = entries2.iter().map(|(k, _v)| k).cloned().collect();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend::<_, i32>(entries1);
    het_map.extend::<_, String>(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<Key<usize, String>> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys5() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<i32>().unwrap();
    let mut iter = map.keys();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys6() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.keys();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys7() {
    let expected: Vec<Key<usize, i32>> = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<i32>().unwrap();
    let result: Vec<Key<usize, i32>> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_keys8() {
    let expected: Vec<Key<usize, String>> = std::vec![];
    let mut het_map = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<Key<usize, String>> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values1() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries1);
    het_map.extend(entries2);

    let map = het_map.get_map::<i32>().unwrap();
    let mut iter = map.values();

    assert_eq!(iter.next(), Some(&20_i32));
    assert_eq!(iter.next(), Some(&30_i32));
    assert_eq!(iter.next(), Some(&40_i32));
    assert_eq!(iter.next(), Some(&50_i32));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values2() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries1);
    het_map.extend(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.values();

    assert_eq!(iter.next(), Some(&String::from("foo")));
    assert_eq!(iter.next(), Some(&String::from("bar")));
    assert_eq!(iter.next(), Some(&String::from("baz")));
    assert_eq!(iter.next(), Some(&String::from("quux")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values3() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected: Vec<i32> = entries1.iter().map(|(_k, v)| v).cloned().collect();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries1);
    het_map.extend(entries2);

    let map = het_map.get_map::<i32>().unwrap();
    let result: Vec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values4() {
    let entries1 = std::vec![
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 30_i32),
        (Key::new(3_usize), 40_i32),
        (Key::new(4_usize), 50_i32),
    ];
    let entries2 = std::vec![
        (Key::new(1_usize), String::from("foo")),
        (Key::new(2_usize), String::from("bar")),
        (Key::new(3_usize), String::from("baz")),
        (Key::new(4_usize), String::from("quux")),
    ];
    let expected: Vec<String> = entries2.iter().map(|(_k, v)| v).cloned().collect();

    let mut het_map = HeterogeneousHashMap::new();
    het_map.extend(entries1);
    het_map.extend(entries2);

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<String> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values5() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.values();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values6() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let mut iter = map.values();

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values7() {
    let expected: Vec<i32> = std::vec![];
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<i32>().unwrap();
    let result: Vec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_values8() {
    let expected: Vec<String> = std::vec![];
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    let map = het_map.get_map::<String>().unwrap();
    let result: Vec<String> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_heterogeneous_hash_map_two_types_clear1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<i32>();
    het_map.insert_type::<Box<dyn any::Any>>();
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(map.is_empty());

        map.clear();

        assert!(map.is_empty());
    }
    {
        let map = het_map.get_map_mut::<Box<dyn any::Any>>().unwrap();
        assert!(map.is_empty());

        map.clear();

        assert!(map.is_empty());
    }
}

#[test]
fn test_heterogeneous_hash_map_two_types_clear2() {
    let mut het_map = HeterogeneousHashMap::new();

    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    assert_eq!(het_map.len::<i32>(), Some(0));
    assert_eq!(het_map.len::<String>(), Some(0));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(map.is_empty());

        map.extend([(Key::new(1_usize), i32::MIN), (Key::new(2_usize), 0_i32), (Key::new(3_usize), i32::MAX)]);
    }
    {
        let map = het_map.get_map_mut::<String>().unwrap();
        assert!(map.is_empty());

        map.extend([
            (Key::new(1_usize), i32::MIN.to_string()),
            (Key::new(2_usize), 0_i32.to_string()),
            (Key::new(3_usize), i32::MAX.to_string()),
        ]);
    }
    assert_eq!(het_map.len::<i32>(), Some(3));
    assert_eq!(het_map.len::<String>(), Some(3));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(!map.is_empty());

        map.clear();
    }
    assert_eq!(het_map.len::<i32>(), Some(0));
    assert_eq!(het_map.len::<String>(), Some(3));
}

#[test]
fn test_heterogeneous_hash_map_two_types_clear3() {
    let mut het_map = HeterogeneousHashMap::new();

    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    assert_eq!(het_map.len::<i32>(), Some(0));
    assert_eq!(het_map.len::<String>(), Some(0));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(map.is_empty());

        map.extend([(Key::new(1_usize), i32::MIN), (Key::new(2_usize), 0_i32), (Key::new(3_usize), i32::MAX)]);
    }
    {
        let map = het_map.get_map_mut::<String>().unwrap();
        assert!(map.is_empty());

        map.extend([
            (Key::new(1_usize), i32::MIN.to_string()),
            (Key::new(2_usize), 0_i32.to_string()),
            (Key::new(3_usize), i32::MAX.to_string()),
        ]);
    }
    assert_eq!(het_map.len::<i32>(), Some(3));
    assert_eq!(het_map.len::<String>(), Some(3));
    {
        let map = het_map.get_map_mut::<String>().unwrap();
        assert!(!map.is_empty());

        map.clear();
    }
    assert_eq!(het_map.len::<i32>(), Some(3));
    assert_eq!(het_map.len::<String>(), Some(0));
}

#[test]
fn test_heterogeneous_hash_map_two_types_clear4() {
    let mut het_map = HeterogeneousHashMap::new();

    assert_eq!(het_map.len::<i32>(), None);

    het_map.insert_type::<i32>();
    het_map.insert_type::<String>();

    assert_eq!(het_map.len::<i32>(), Some(0));
    assert_eq!(het_map.len::<String>(), Some(0));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(map.is_empty());

        map.extend([(Key::new(1_usize), i32::MIN), (Key::new(2_usize), 0_i32), (Key::new(3_usize), i32::MAX)]);
    }
    {
        let map = het_map.get_map_mut::<String>().unwrap();
        assert!(map.is_empty());

        map.extend([
            (Key::new(1_usize), i32::MIN.to_string()),
            (Key::new(2_usize), 0_i32.to_string()),
            (Key::new(3_usize), i32::MAX.to_string()),
        ]);
    }
    assert_eq!(het_map.len::<i32>(), Some(3));
    assert_eq!(het_map.len::<String>(), Some(3));
    {
        let map = het_map.get_map_mut::<i32>().unwrap();
        assert!(!map.is_empty());

        map.clear();
    }
    {
        let map = het_map.get_map_mut::<String>().unwrap();
        assert!(!map.is_empty());

        map.clear();
    }
    assert_eq!(het_map.len::<i32>(), Some(0));
    assert_eq!(het_map.len::<String>(), Some(0));
}
