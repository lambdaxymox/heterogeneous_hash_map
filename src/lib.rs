#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod entry;
mod error;
mod heterogeneous_hash_map;
mod homogeneous_hash_map;
mod iterator;
mod key;
mod metadata;

pub use crate::entry::*;
pub use crate::error::*;
pub use crate::heterogeneous_hash_map::*;
pub use crate::homogeneous_hash_map::*;
pub use crate::iterator::*;
pub use crate::key::Key;
pub use crate::metadata::*;
