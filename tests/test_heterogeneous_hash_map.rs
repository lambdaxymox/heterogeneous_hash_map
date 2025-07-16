#![no_std]
extern crate std;

use heterogeneous_hash_map::{HeterogeneousHashMap, Key};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use core::any;
use std::boxed::Box;
use std::string::String;

#[test]
fn test_heterogeneous_hash_map_contains_type_empty() {
    let het_map = HeterogeneousHashMap::new();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
    assert!(!het_map.contains_type::<i32>());
    assert!(!het_map.contains_type::<i64>());
    assert!(!het_map.contains_type::<isize>());
    assert!(!het_map.contains_type::<u8>());
    assert!(!het_map.contains_type::<u16>());
    assert!(!het_map.contains_type::<u32>());
    assert!(!het_map.contains_type::<u64>());
    assert!(!het_map.contains_type::<usize>());
    assert!(!het_map.contains_type::<f32>());
    assert!(!het_map.contains_type::<f64>());
    assert!(!het_map.contains_type::<char>());
    assert!(!het_map.contains_type::<String>());
    assert!(!het_map.contains_type::<&str>());
    assert!(!het_map.contains_type::<Box<dyn any::Any>>());
}

#[test]
fn test_heterogeneous_hash_map_contains_type() {
    let mut het_map = HeterogeneousHashMap::new();

    het_map.insert_type::<()>();
    het_map.insert_type::<bool>();
    het_map.insert_type::<i8>();
    het_map.insert_type::<i16>();
    het_map.insert_type::<i32>();
    het_map.insert_type::<i64>();
    het_map.insert_type::<isize>();
    het_map.insert_type::<u8>();
    het_map.insert_type::<u16>();
    het_map.insert_type::<u32>();
    het_map.insert_type::<u64>();
    het_map.insert_type::<usize>();
    het_map.insert_type::<f32>();
    het_map.insert_type::<f64>();
    het_map.insert_type::<char>();
    het_map.insert_type::<String>();
    het_map.insert_type::<&str>();
    het_map.insert_type::<Box<dyn any::Any>>();

    assert!(het_map.contains_type::<()>());
    assert!(het_map.contains_type::<bool>());
    assert!(het_map.contains_type::<i8>());
    assert!(het_map.contains_type::<i16>());
    assert!(het_map.contains_type::<i32>());
    assert!(het_map.contains_type::<i64>());
    assert!(het_map.contains_type::<isize>());
    assert!(het_map.contains_type::<u8>());
    assert!(het_map.contains_type::<u16>());
    assert!(het_map.contains_type::<u32>());
    assert!(het_map.contains_type::<u64>());
    assert!(het_map.contains_type::<usize>());
    assert!(het_map.contains_type::<f32>());
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());
}

#[test]
fn test_heterogeneous_hash_map_single_type_zst1() {
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
fn test_heterogeneous_hash_map_single_type_zst2() {
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
fn test_heterogeneous_hash_map_single_type_zst3() {
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
fn test_heterogeneous_hash_map_single_type_zst4() {
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
