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
fn test_heterogeneous_hash_map_get_map_empty() {
    let het_map = HeterogeneousHashMap::new();

    assert!(het_map.get_map::<()>().is_none());
    assert!(het_map.get_map::<bool>().is_none());
    assert!(het_map.get_map::<i8>().is_none());
    assert!(het_map.get_map::<i16>().is_none());
    assert!(het_map.get_map::<i32>().is_none());
    assert!(het_map.get_map::<i64>().is_none());
    assert!(het_map.get_map::<isize>().is_none());
    assert!(het_map.get_map::<u8>().is_none());
    assert!(het_map.get_map::<u16>().is_none());
    assert!(het_map.get_map::<u32>().is_none());
    assert!(het_map.get_map::<u64>().is_none());
    assert!(het_map.get_map::<usize>().is_none());
    assert!(het_map.get_map::<f32>().is_none());
    assert!(het_map.get_map::<f64>().is_none());
    assert!(het_map.get_map::<char>().is_none());
    assert!(het_map.get_map::<String>().is_none());
    assert!(het_map.get_map::<&str>().is_none());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_none());
}

#[test]
fn test_heterogeneous_hash_map_get_map() {
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

    assert!(het_map.get_map::<()>().is_some());
    assert!(het_map.get_map::<bool>().is_some());
    assert!(het_map.get_map::<i8>().is_some());
    assert!(het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());
}

#[test]
fn test_heterogeneous_hash_map_remove_type_contains_type() {
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

    assert!(het_map.get_map::<()>().is_some());
    assert!(het_map.get_map::<bool>().is_some());
    assert!(het_map.get_map::<i8>().is_some());
    assert!(het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<()>();

    assert!(!het_map.contains_type::<()>());
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

    het_map.remove_type::<bool>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
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

    het_map.remove_type::<i8>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
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

    het_map.remove_type::<i16>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
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

    het_map.remove_type::<i32>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
    assert!(!het_map.contains_type::<i32>());
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

    het_map.remove_type::<i64>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
    assert!(!het_map.contains_type::<i32>());
    assert!(!het_map.contains_type::<i64>());
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

    het_map.remove_type::<isize>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
    assert!(!het_map.contains_type::<i32>());
    assert!(!het_map.contains_type::<i64>());
    assert!(!het_map.contains_type::<isize>());
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

    het_map.remove_type::<u8>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
    assert!(!het_map.contains_type::<i32>());
    assert!(!het_map.contains_type::<i64>());
    assert!(!het_map.contains_type::<isize>());
    assert!(!het_map.contains_type::<u8>());
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

    het_map.remove_type::<u16>();

    assert!(!het_map.contains_type::<()>());
    assert!(!het_map.contains_type::<bool>());
    assert!(!het_map.contains_type::<i8>());
    assert!(!het_map.contains_type::<i16>());
    assert!(!het_map.contains_type::<i32>());
    assert!(!het_map.contains_type::<i64>());
    assert!(!het_map.contains_type::<isize>());
    assert!(!het_map.contains_type::<u8>());
    assert!(!het_map.contains_type::<u16>());
    assert!(het_map.contains_type::<u32>());
    assert!(het_map.contains_type::<u64>());
    assert!(het_map.contains_type::<usize>());
    assert!(het_map.contains_type::<f32>());
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<u32>();

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
    assert!(het_map.contains_type::<u64>());
    assert!(het_map.contains_type::<usize>());
    assert!(het_map.contains_type::<f32>());
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<u64>();

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
    assert!(het_map.contains_type::<usize>());
    assert!(het_map.contains_type::<f32>());
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<usize>();

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
    assert!(het_map.contains_type::<f32>());
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<f32>();

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
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<f64>();

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
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<char>();

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
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<String>();

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
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<&str>();

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
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<Box<dyn any::Any>>();

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
fn test_heterogeneous_hash_map_remove_type_get_map() {
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

    assert!(het_map.get_map::<()>().is_some());
    assert!(het_map.get_map::<bool>().is_some());
    assert!(het_map.get_map::<i8>().is_some());
    assert!(het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<()>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(het_map.get_map::<bool>().is_some());
    assert!(het_map.get_map::<i8>().is_some());
    assert!(het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<bool>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(het_map.get_map::<i8>().is_some());
    assert!(het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<i8>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<i16>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<i32>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<i64>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<isize>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<u8>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<u16>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<u32>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<u64>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(het_map.get_map::<usize>().is_some());
    assert!(het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<usize>();

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
    assert!(het_map.contains_type::<f32>());
    assert!(het_map.contains_type::<f64>());
    assert!(het_map.contains_type::<char>());
    assert!(het_map.contains_type::<String>());
    assert!(het_map.contains_type::<&str>());
    assert!(het_map.contains_type::<Box<dyn any::Any>>());

    het_map.remove_type::<f32>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(!het_map.get_map::<usize>().is_some());
    assert!(!het_map.get_map::<f32>().is_some());
    assert!(het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<f64>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(!het_map.get_map::<usize>().is_some());
    assert!(!het_map.get_map::<f32>().is_some());
    assert!(!het_map.get_map::<f64>().is_some());
    assert!(het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<char>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(!het_map.get_map::<usize>().is_some());
    assert!(!het_map.get_map::<f32>().is_some());
    assert!(!het_map.get_map::<f64>().is_some());
    assert!(!het_map.get_map::<char>().is_some());
    assert!(het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<String>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(!het_map.get_map::<usize>().is_some());
    assert!(!het_map.get_map::<f32>().is_some());
    assert!(!het_map.get_map::<f64>().is_some());
    assert!(!het_map.get_map::<char>().is_some());
    assert!(!het_map.get_map::<String>().is_some());
    assert!(het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<&str>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(!het_map.get_map::<usize>().is_some());
    assert!(!het_map.get_map::<f32>().is_some());
    assert!(!het_map.get_map::<f64>().is_some());
    assert!(!het_map.get_map::<char>().is_some());
    assert!(!het_map.get_map::<String>().is_some());
    assert!(!het_map.get_map::<&str>().is_some());
    assert!(het_map.get_map::<Box<dyn any::Any>>().is_some());

    het_map.remove_type::<Box<dyn any::Any>>();

    assert!(!het_map.get_map::<()>().is_some());
    assert!(!het_map.get_map::<bool>().is_some());
    assert!(!het_map.get_map::<i8>().is_some());
    assert!(!het_map.get_map::<i16>().is_some());
    assert!(!het_map.get_map::<i32>().is_some());
    assert!(!het_map.get_map::<i64>().is_some());
    assert!(!het_map.get_map::<isize>().is_some());
    assert!(!het_map.get_map::<u8>().is_some());
    assert!(!het_map.get_map::<u16>().is_some());
    assert!(!het_map.get_map::<u32>().is_some());
    assert!(!het_map.get_map::<u64>().is_some());
    assert!(!het_map.get_map::<usize>().is_some());
    assert!(!het_map.get_map::<f32>().is_some());
    assert!(!het_map.get_map::<f64>().is_some());
    assert!(!het_map.get_map::<char>().is_some());
    assert!(!het_map.get_map::<String>().is_some());
    assert!(!het_map.get_map::<&str>().is_some());
    assert!(!het_map.get_map::<Box<dyn any::Any>>().is_some());
}
