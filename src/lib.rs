#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod key;
mod iterator;
mod metadata;
mod homogeneous_hash_map;
mod heterogeneous_hash_map;

pub use crate::heterogeneous_hash_map::*;
pub use crate::homogeneous_hash_map::*;
pub use crate::iterator::*;
pub use crate::key::Key;
pub use crate::metadata::*;
pub use crate::error::*;
