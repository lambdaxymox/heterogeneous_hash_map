#![no_std]
extern crate std;

use heterogeneous_hash_map::HeterogeneousHashMap;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use core::any;
use std::boxed::Box;
use std::string::String;

#[test]
fn test_heterogeneous_hash_map_contains_type_empty() {
    let hmap = HeterogeneousHashMap::new();

    assert!(!hmap.contains_type::<()>());
    assert!(!hmap.contains_type::<bool>());
    assert!(!hmap.contains_type::<i8>());
    assert!(!hmap.contains_type::<i16>());
    assert!(!hmap.contains_type::<i32>());
    assert!(!hmap.contains_type::<i64>());
    assert!(!hmap.contains_type::<isize>());
    assert!(!hmap.contains_type::<u8>());
    assert!(!hmap.contains_type::<u16>());
    assert!(!hmap.contains_type::<u32>());
    assert!(!hmap.contains_type::<u64>());
    assert!(!hmap.contains_type::<usize>());
    assert!(!hmap.contains_type::<f32>());
    assert!(!hmap.contains_type::<f64>());
    assert!(!hmap.contains_type::<char>());
    assert!(!hmap.contains_type::<String>());
    assert!(!hmap.contains_type::<&str>());
    assert!(!hmap.contains_type::<Box<dyn any::Any>>());
}

#[test]
fn test_heterogeneous_hash_map_contains_type() {
    let mut hmap = HeterogeneousHashMap::new();

    hmap.insert_type::<()>();
    hmap.insert_type::<bool>();
    hmap.insert_type::<i8>();
    hmap.insert_type::<i16>();
    hmap.insert_type::<i32>();
    hmap.insert_type::<i64>();
    hmap.insert_type::<isize>();
    hmap.insert_type::<u8>();
    hmap.insert_type::<u16>();
    hmap.insert_type::<u32>();
    hmap.insert_type::<u64>();
    hmap.insert_type::<usize>();
    hmap.insert_type::<f32>();
    hmap.insert_type::<f64>();
    hmap.insert_type::<char>();
    hmap.insert_type::<String>();
    hmap.insert_type::<&str>();
    hmap.insert_type::<Box<dyn any::Any>>();

    assert!(hmap.contains_type::<()>());
    assert!(hmap.contains_type::<bool>());
    assert!(hmap.contains_type::<i8>());
    assert!(hmap.contains_type::<i16>());
    assert!(hmap.contains_type::<i32>());
    assert!(hmap.contains_type::<i64>());
    assert!(hmap.contains_type::<isize>());
    assert!(hmap.contains_type::<u8>());
    assert!(hmap.contains_type::<u16>());
    assert!(hmap.contains_type::<u32>());
    assert!(hmap.contains_type::<u64>());
    assert!(hmap.contains_type::<usize>());
    assert!(hmap.contains_type::<f32>());
    assert!(hmap.contains_type::<f64>());
    assert!(hmap.contains_type::<char>());
    assert!(hmap.contains_type::<String>());
    assert!(hmap.contains_type::<&str>());
    assert!(hmap.contains_type::<Box<dyn any::Any>>());
}
