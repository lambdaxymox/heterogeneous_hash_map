#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(
        allocator_api,
    )
)]
#![no_std]
extern crate std;

mod test_heterogeneous_hash_map;
mod test_homogeneous_hash_map;
