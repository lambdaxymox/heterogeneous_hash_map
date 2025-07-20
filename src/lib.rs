#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod try_reserve_error;

use crate::try_reserve_error::{TryReserveError, TryReserveErrorKind};

use core::any;
use core::cmp;
use core::fmt;
use core::iter;
use core::marker;
use core::mem;
use core::ops;
use core::borrow::Borrow;
use alloc_crate::vec::Vec;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use hashbrown::hash_map;

/// A typed key type for heterogeneous hash maps.
///
/// This implements the hierarchical hashing structure of the heterogeneous hash map in a
/// type-safe manner: the first level of hashing is given by the type of the key, and the
/// second level of hashing is done by the value of the key.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.insert_type::<i32>();
///
/// let key: Key<usize, i32> = Key::new(1);
///
/// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
/// assert!(!het_map.contains_key::<char, _>(&Key::new(1)));
/// assert_eq!(het_map.get::<i32, _>(&Key::new(1)),  None);
/// assert_eq!(het_map.get::<char, _>(&Key::new(1)), None);
///
/// het_map.insert::<i32>(key, 3_i32);
///
/// assert!(het_map.contains_key::<i32, _>(&Key::new(1)));
/// assert!(!het_map.contains_key::<char, _>(&Key::new(1)));
/// assert_eq!(het_map.get::<i32, _>(&Key::new(1)),  Some(&3_i32));
/// assert_eq!(het_map.get::<char, _>(&Key::new(1)), None);
/// ```
#[repr(transparent)]
#[derive(Debug)]
pub struct Key<K, T> {
    id: K,
    _marker: marker::PhantomData<T>,
}

impl<K, T> Key<K, T> {
    /// Constructs a new typed key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// # use core::any::Any;
    /// #
    /// let key: Key<usize, Box<dyn Any>> = Key::new(usize::MAX);
    /// ```
    #[inline]
    pub const fn new(id: K) -> Self {
        Self {
            id,
            _marker: marker::PhantomData,
        }
    }

    /// Returns the underlying numerical value of the typed key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// # use core::any::Any;
    /// #
    /// let key: Key<usize, Box<dyn Any>> = Key::new(usize::MAX);
    ///
    /// assert_eq!(key.id(), &usize::MAX);
    /// ```
    #[inline]
    pub const fn id(&self) -> &K {
        &self.id
    }
}

impl<K, T> From<K> for Key<K, T> {
    fn from(id: K) -> Self {
        Self::new(id)
    }
}

impl<K, T> Default for Key<K, T>
where
    K: Default,
{
    fn default() -> Self {
        Self::new(K::default())
    }
}

impl<K, T> Clone for Key<K, T>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.id.clone())
    }
}

impl<K, T> Copy for Key<K, T>
where
    K: Copy,
{
}

impl<K, T> hash::Hash for Key<K, T>
where
    K: hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl<K, T> PartialEq for Key<K, T>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<K, T> Eq for Key<K, T>
where
    K: Eq,
{
}

impl<K, T> PartialOrd for Key<K, T>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&self.id, &other.id)
    }
}

impl<K, T> Ord for Key<K, T>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&self.id, &other.id)
    }
}

impl<K, T> fmt::Display for Key<K, T>
where
    K: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{id}", id = self.id)
    }
}

/// An immutable iterator over the entries of a hash map.
///
/// Iterators are created by the [`HomogeneousHashMap::iter`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.extend([
///     (Key::new(1), 2_i32),
///     (Key::new(2), 3_i32),
///     (Key::new(3), 5_i32),
/// ]);
/// let expected = vec![2_i32, 3_i32, 5_i32];
/// let result = {
///     let map = het_map.get_map::<i32>().unwrap();
///     let mut _result = Vec::from_iter(map.iter().map(|(_k, v)| v.clone()));
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
pub struct Iter<'a, K, T> {
    iter: opaque::index_map::map::Iter<'a, Key<K, T>, T>,
}

impl<'a, K, T> Iter<'a, K, T> {
    /// Constructs a new immutable iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::Iter<'a, Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Iterator for Iter<'a, K, T> {
    type Item = (&'a Key<K, T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, T> DoubleEndedIterator for Iter<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, K, T> ExactSizeIterator for Iter<'a, K, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, T> iter::FusedIterator for Iter<'_, K, T> {}

impl<'a, K, T> Clone for Iter<'a, K, T> {
    fn clone(&self) -> Self {
        Iter { iter: self.iter.clone() }
    }
}

impl<'a, K, T> Default for Iter<'a, K, T> {
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

/// A mutable iterator over the entries of a hash map.
///
/// Mutable iterators are created by the [`HomogeneousHashMap::iter_mut`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.extend([
///     (Key::new(1), 2_i32),
///     (Key::new(2), 3_i32),
///     (Key::new(3), 5_i32),
/// ]);
/// let expected = vec![2_i32, 3_i32, 5_i32];
/// let result = {
///     let map = het_map.get_map_mut::<i32>().unwrap();
///     let mut _result = Vec::from_iter(map.iter_mut().map(|(_k, v)| v.clone()));
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
pub struct IterMut<'a, K, T> {
    iter: opaque::index_map::map::IterMut<'a, Key<K, T>, T>,
}

impl<'a, K, T> IterMut<'a, K, T> {
    /// Constructs a new mutable iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::IterMut<'a, Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Iterator for IterMut<'a, K, T> {
    type Item = (&'a Key<K, T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, T> DoubleEndedIterator for IterMut<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, K, T> ExactSizeIterator for IterMut<'a, K, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, T> iter::FusedIterator for IterMut<'a, K, T> {}

impl<'a, K, T> fmt::Debug for IterMut<'a, K, T>
where
    K: fmt::Debug,
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<'a, K, T> Default for IterMut<'a, K, T> {
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

/// An iterator over the keys of the hash map.
///
/// Key iterators are created using the [`HomogeneousHashMap::keys`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.extend([
///     (Key::new(2), 'b'),
///     (Key::new(3), 'c'),
///     (Key::new(5), 'e'),
///     (Key::new(7), 'g'),
/// ]);
/// let expected = vec![Key::new(2), Key::new(3), Key::new(5), Key::new(7)];
/// let result = {
///     let map = het_map.get_map::<char>().unwrap();
///     let mut _result = Vec::from_iter(map.keys().cloned());
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
pub struct Keys<'a, K, T> {
    iter: opaque::index_map::map::Keys<'a, Key<K, T>, T>,
}

impl<'a, K, T> Keys<'a, K, T> {
    /// Constructs a new key iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::Keys<'a, Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Clone for Keys<'a, K, T> {
    fn clone(&self) -> Self {
        Keys { iter: self.iter.clone() }
    }
}

impl<'a, K, T> Iterator for Keys<'a, K, T> {
    type Item = &'a Key<K, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, T> DoubleEndedIterator for Keys<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, K, T> ExactSizeIterator for Keys<'a, K, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, T> iter::FusedIterator for Keys<'a, K, T> {}

impl<'a, K, T> fmt::Debug for Keys<'a, K, T>
where
    K: fmt::Debug,
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<'a, K, T> Default for Keys<'a, K, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

/// An immutable iterator over the values of the hash map.
///
/// Value iterators are created using the [`HomogeneousHashMap::values`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.extend([
///     (Key::new(2), 'b'),
///     (Key::new(3), 'c'),
///     (Key::new(5), 'e'),
///     (Key::new(7), 'g'),
/// ]);
/// let expected = vec!['b', 'c', 'e', 'g'];
/// let result = {
///     let map = het_map.get_map::<char>().unwrap();
///     let mut _result = Vec::from_iter(map.values().cloned());
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
pub struct Values<'a, K, T> {
    iter: opaque::index_map::map::Values<'a, Key<K, T>, T>,
}

impl<'a, K, T> Values<'a, K, T> {
    /// Constructs a new immutable value iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::Values<'a, Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Iterator for Values<'a, K, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, T> DoubleEndedIterator for Values<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, K, T> ExactSizeIterator for Values<'a, K, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, T> iter::FusedIterator for Values<'a, K, T> {}

impl<'a, K, T> Clone for Values<'a, K, T> {
    fn clone(&self) -> Self {
        Values { iter: self.iter.clone() }
    }
}

impl<'a, K, T> fmt::Debug for Values<'a, K, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K, T> Default for Values<'a, K, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

/// A mutable iterator over the values of the hash map.
///
/// Mutable value iterators are created using the [`HomogeneousHashMap::values_mut`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.extend([
///     (Key::new(2), 'b'),
///     (Key::new(3), 'c'),
///     (Key::new(5), 'e'),
///     (Key::new(7), 'g'),
/// ]);
/// let expected = vec!['b', 'c', 'e', 'g'];
/// let result = {
///     let map = het_map.get_map_mut::<char>().unwrap();
///     let mut _result = Vec::from_iter(map.values_mut().map(|v| v.clone()));
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
pub struct ValuesMut<'a, K, T> {
    iter: opaque::index_map::map::ValuesMut<'a, Key<K, T>, T>,
}

impl<'a, K, T> ValuesMut<'a, K, T> {
    /// Constructs a new mutable value iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::ValuesMut<'a, Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Iterator for ValuesMut<'a, K, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, T> DoubleEndedIterator for ValuesMut<'a, K, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, K, T> ExactSizeIterator for ValuesMut<'a, K, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, T> iter::FusedIterator for ValuesMut<'a, K, T> {}

impl<'a, K, T> fmt::Debug for ValuesMut<'a, K, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<'a, K, T> Default for ValuesMut<'a, K, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

/// A draining iterator over the entries of a hash map.
///
/// Draining iterators are created by the [`HomogeneousHashMap::drain`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// let expected = vec![
///     (Key::new(2), String::from("foo")),
///     (Key::new(3), String::from("bar")),
///     (Key::new(5), String::from("baz")),
/// ];
/// het_map.extend(expected.clone());
///
/// assert_eq!(het_map.len::<String>(), Some(3));
///
/// let result = {
///     let map = het_map.get_map_mut::<String>().unwrap();
///     let mut _result = vec![];
///     for entry in map.drain() {
///         _result.push(entry);
///     }
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(het_map.len::<String>(), Some(0));
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
pub struct Drain<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    iter: opaque::index_map::map::Drain<'a, Key<K, T>, T>,
}

impl<'a, K, T> Drain<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs a new draining iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::Drain<'a, Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, K, T> Iterator for Drain<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    type Item = (Key<K, T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, T> DoubleEndedIterator for Drain<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, K, T> ExactSizeIterator for Drain<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, T> iter::FusedIterator for Drain<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
}

impl<'a, K, T> fmt::Debug for Drain<'a, K, T>
where
    K: any::Any + fmt::Debug,
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

/// An extracting iterator over the entries of a hash map.
///
/// Extracting iterators are created by the [`HomogeneousHashMap::extract_if`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// fn is_prime(n: i32) -> bool {
///     if n < 2 {
///         return false;
///     }
///     let mut i = 2;
///     while i * i <= n {
///         if n % i == 0 {
///             return false;
///         }
///         i += 1;
///     }
///     true
/// }
///
/// let mut het_map = HeterogeneousHashMap::new();
/// het_map.extend([
///     (Key::new(0), 1_i32), (Key::new(1), 2_i32),  (Key::new(2),  3_i32),  (Key::new(3), 4_i32),
///     (Key::new(4), 5_i32), (Key::new(5), 6_i32),  (Key::new(6),  7_i32),  (Key::new(7), 8_i32),
///     (Key::new(8), 9_i32), (Key::new(9), 10_i32), (Key::new(10), 11_i32), (Key::new(11), 12_i32),
/// ]);
/// let expected = vec![2_i32, 3_i32, 5_i32, 7_i32, 11_i32];
/// let result = {
///     let mut _result: Vec<i32> = het_map
///         .get_map_mut::<i32>()
///         .unwrap()
///         .extract_if(|_k, v| is_prime(*v))
///         .map(|(_k, v)| v)
///         .collect();
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
///
/// for i in result.iter() {
///     assert!(is_prime(*i));
/// }
/// ```
pub struct ExtractIf<'a, K, T, F>
where
    K: any::Any,
    T: any::Any,
    F: FnMut(&Key<K, T>, &mut T) -> bool,
{
    iter: opaque::index_map::map::ExtractIf<'a, Key<K, T>, T, F>,
}

impl<'a, K, T, F> ExtractIf<'a, K, T, F>
where
    K: any::Any,
    T: any::Any,
    F: FnMut(&Key<K, T>, &mut T) -> bool,
{
    /// Constructs a new extracting iterator.
    #[inline]
    const fn new(iter: opaque::index_map::map::ExtractIf<'a, Key<K, T>, T, F>) -> ExtractIf<'a, K, T, F> {
        ExtractIf { iter }
    }
}

impl<'a, K, T, F> Iterator for ExtractIf<'a, K, T, F>
where
    K: any::Any,
    T: any::Any,
    F: FnMut(&Key<K, T>, &mut T) -> bool,
{
    type Item = (Key<K, T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, T, F> iter::FusedIterator for ExtractIf<'_, K, T, F>
where
    K: any::Any,
    T: any::Any,
    F: FnMut(&Key<K, T>, &mut T) -> bool,
{
}

impl<'a, K, T, F> fmt::Debug for ExtractIf<'a, K, T, F>
where
    K: any::Any + fmt::Debug,
    T: any::Any + fmt::Debug,
    F: FnMut(&Key<K, T>, &mut T) -> bool,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("ExtractIf").finish_non_exhaustive()
    }
}

/// A hash map containing all values of a given type in a heterogeneous hash map.
///
/// This type acts similarly to a standard library [`HashMap`]. This type of hash map can also
/// be constructed for testing purposes, but these are primarily accessed by calling the
/// [`HeterogeneousHashMap::get_map`], [`HeterogeneousHashMap::get_map_mut`],
/// [`HeterogeneousHashMap::get_map_unchecked`], and [`HeterogeneousHashMap::get_map_mut_unchecked`]
/// methods.
///
/// # Examples
///
/// Getting a hash map from a heterogeneous hash map.
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// #
/// let mut het_map = HeterogeneousHashMap::new();
/// let expected = vec![
///     (Key::new(0), 2_u32),
///     (Key::new(1), 3_u32),
///     (Key::new(2), 5_u32),
///     (Key::new(3), 7_u32),
///     (Key::new(4), 11_u32),
///     (Key::new(5), 13_u32),
/// ];
/// het_map.extend(expected.clone());
/// let map = het_map.get_map::<u32>().unwrap();
/// let result = {
///     let mut _result = Vec::from_iter(map.iter().map(|(k, v)| (k.clone(), v.clone())));
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result.len(), expected.len());
/// assert_eq!(result, expected);
/// ```
#[cfg(feature = "std")]
#[repr(transparent)]
pub struct HomogeneousHashMap<K, T, S = hash::RandomState>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    inner: opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>,
}

#[cfg(not(feature = "std"))]
#[repr(transparent)]
pub struct HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    inner: opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>,
}

impl<K, T, S> HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Constructs a new hash map.
    #[inline]
    const fn from_inner(inner: opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>) -> Self {
        Self { inner }
    }

    /// Constructs a new hash map.
    #[inline]
    const fn from_inner_ref(map: &opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>) -> &Self {
        unsafe { &*(map as *const opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S> as *const Self) }
    }

    /// Constructs a new hash map.
    #[inline]
    const fn from_inner_ref_mut(map: &mut opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>) -> &mut Self {
        unsafe { &mut *(map as *const opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S> as *mut Self) }
    }
}

impl<K, T, S> HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Constructs a new hash map with the given hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HomogeneousHashMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::with_hasher(RandomState::new());
    ///
    /// assert!(map.is_empty());
    /// ```
    #[inline]
    pub fn with_hasher(build_hasher: S) -> Self {
        Self {
            inner: opaque::index_map::TypeProjectedIndexMap::with_hasher(build_hasher),
        }
    }

    /// Constructs a new hash map with at least the given capacity with the given hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
    /// # use std::hash::RandomState;
    /// #
    /// let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::with_capacity_and_hasher(3, RandomState::new());
    ///
    /// assert_eq!(map.len(), 0);
    /// assert!(map.capacity() >= 3);
    /// let old_capacity = map.capacity();
    ///
    /// map.insert(Key::new(0), 1_i32);
    /// map.insert(Key::new(1), 2_i32);
    /// map.insert(Key::new(2), 3_i32);
    ///
    /// assert_eq!(map.len(), 3);
    /// assert!(map.capacity() >= old_capacity);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: S) -> Self {
        Self {
            inner: opaque::index_map::TypeProjectedIndexMap::with_capacity_and_hasher(capacity, build_hasher),
        }
    }
}

#[cfg(feature = "std")]
impl<K, T> HomogeneousHashMap<K, T, hash::RandomState>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs a new hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HomogeneousHashMap;
    /// #
    /// let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::new();
    ///
    /// assert!(map.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::with_hasher(hash::RandomState::new())
    }

    /// Constructs a new hash map with at least the given capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
    /// #
    /// let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::with_capacity(3);
    ///
    /// assert_eq!(map.len(), 0);
    /// assert!(map.capacity() >= 3);
    /// let old_capacity = map.capacity();
    ///
    /// map.insert(Key::new(0), 1_i32);
    /// map.insert(Key::new(1), 2_i32);
    /// map.insert(Key::new(2), 3_i32);
    ///
    /// assert_eq!(map.len(), 3);
    /// assert!(map.capacity() >= old_capacity);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, hash::RandomState::new())
    }
}

impl<K, T, S> HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns the capacity of the hash map.
    ///
    /// The **capacity** of a hash map is the maximum number of entries it can contain without
    /// reallocating memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type_with_capacity::<String>(3);
    /// let old_capacity = {
    ///     let map = het_map.get_map::<String>().unwrap();
    ///     assert!(map.capacity() >= 3);
    ///
    ///     map.capacity()
    /// };
    /// het_map.extend([
    ///     (Key::new(0), String::from("foo")),
    ///     (Key::new(1), String::from("bar")),
    ///     (Key::new(2), String::from("baz")),
    /// ]);
    /// let map = het_map.get_map_mut::<String>().unwrap();
    ///
    /// assert_eq!(map.capacity(), old_capacity);
    ///
    /// map.insert(Key::new(3), String::from("quux"));
    ///
    /// assert!(map.capacity() >= old_capacity);
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns the length of the hash map.
    ///
    /// The **length** of a hash map is the number of entries it currently contains.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type_with_capacity::<String>(3);
    /// {
    ///     let map = het_map.get_map::<String>().unwrap();
    ///     assert_eq!(map.len(), 0);
    /// }
    /// het_map.extend([
    ///     (Key::new(0), String::from("foo")),
    ///     (Key::new(1), String::from("bar")),
    ///     (Key::new(2), String::from("baz")),
    /// ]);
    /// let map = het_map.get_map::<String>().unwrap();
    ///
    /// assert_eq!(map.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Determines whether a hash map is empty.
    ///
    /// A hash map is **empty** if it contains no elements, i.e. its length is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type_with_capacity::<String>(3);
    /// {
    ///     let map = het_map.get_map::<String>().unwrap();
    ///     assert!(map.is_empty());
    /// }
    /// het_map.extend([
    ///     (Key::new(0), String::from("foo")),
    ///     (Key::new(1), String::from("bar")),
    ///     (Key::new(2), String::from("baz")),
    /// ]);
    /// let map = het_map.get_map::<String>().unwrap();
    ///
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a reference to the hash map's hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HomogeneousHashMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map: HomogeneousHashMap<usize, String> = HomogeneousHashMap::with_hasher(RandomState::new());
    /// let build_hasher: &RandomState = map.hasher();
    /// ```
    #[inline]
    pub fn hasher(&self) -> &S {
        self.inner.hasher().get_build_hasher()
    }
}

impl<K, T, S> HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Determines whether a hash map contains an equivalent key to the given key.
    ///
    /// This method returns `true` if an equivalent key to the key `key` exists in the hash map.
    /// This method returns `false` if an equivalent key to the key `key` does not exist in the
    /// hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    ///     (Key::new(3), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert!(map.contains_key(&Key::new(1)));
    /// assert!(map.contains_key(&Key::new(2)));
    /// assert!(map.contains_key(&Key::new(3)));
    /// assert!(!map.contains_key(&Key::new(4)));
    /// assert!(!map.contains_key(&Key::new(usize::MAX)));
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.contains_key(key)
    }

    /// Returns a reference to the value with the equivalent key to the given one, if it
    /// exists.
    ///
    /// If an equivalent key to the key `key` exists in the hash map, this method returns
    /// `Some(&value)`, where `value` is the value corresponding to the equivalent key to `key`.
    /// If an equivalent key to the key `key` does not exist in the hash map, this method returns
    /// `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    ///     (Key::new(3), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    /// assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    /// assert_eq!(map.get(&Key::new(3)), Some(&4_f64));
    /// assert_eq!(map.get(&Key::new(4)), None);
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&T>
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.get(key)
    }

    /// Returns a reference to the key-value with the equivalent key to the given one, if it
    /// exists.
    ///
    /// If an equivalent key to the key `key` exists in the hash map, this method returns
    /// `Some((&eq_key, &value))`, where `eq_key` is the equivalent key to the key `key`, and
    /// `value` is the value corresponding to `eq_key`. If an equivalent key to the key `key` does
    /// not exist in the hash map, this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    ///     (Key::new(3), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert_eq!(map.get_key_value(&Key::new(1)), Some((&Key::new(1), &2_f64)));
    /// assert_eq!(map.get_key_value(&Key::new(2)), Some((&Key::new(2), &3_f64)));
    /// assert_eq!(map.get_key_value(&Key::new(3)), Some((&Key::new(3), &4_f64)));
    /// assert_eq!(map.get_key_value(&Key::new(4)), None);
    /// ```
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&Key<K, T>, &T)>
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.get_key_value(key)
    }

    /// Returns a mutable reference to the value with the equivalent key to the given one, if it
    /// exists.
    ///
    /// If an equivalent key to the key `key` exists in the hash map, this method returns
    /// `Some(&mut value)`, where `value` is the value corresponding to the equivalent key to
    /// `key`. If an equivalent key to the key `key` does not exist in the hash map, this method
    /// returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    ///     (Key::new(3), 4_f64),
    /// ]);
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut(&Key::new(1)), Some(&mut 2_f64));
    ///     assert_eq!(map.get_mut(&Key::new(2)), Some(&mut 3_f64));
    ///     assert_eq!(map.get_mut(&Key::new(3)), Some(&mut 4_f64));
    ///     assert_eq!(map.get_mut(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     map.get_mut(&Key::new(1)).map(|v| *v *= 2_f64);
    ///     map.get_mut(&Key::new(2)).map(|v| *v *= 2_f64);
    ///     map.get_mut(&Key::new(3)).map(|v| *v *= 2_f64);
    ///     map.get_mut(&Key::new(4)).map(|v| *v *= 2_f64);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut(&Key::new(1)), Some(&mut 4_f64));
    ///     assert_eq!(map.get_mut(&Key::new(2)), Some(&mut 6_f64));
    ///     assert_eq!(map.get_mut(&Key::new(3)), Some(&mut 8_f64));
    ///     assert_eq!(map.get_mut(&Key::new(4)), None);
    /// }
    /// ```
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut T>
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.get_mut(key)
    }

    /// Returns a reference to the value with the equivalent key to the given one, if it
    /// exists.
    ///
    /// # Panics
    ///
    /// This method panics if an equivalent key to the key `key` does not exist in the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    ///     (Key::new(3), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert_eq!(map.get_unchecked(&Key::new(1)), &2_f64);
    /// assert_eq!(map.get_unchecked(&Key::new(2)), &3_f64);
    /// assert_eq!(map.get_unchecked(&Key::new(3)), &4_f64);
    /// ```
    #[track_caller]
    pub fn get_unchecked<Q>(&self, key: &Q) -> &T
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        &self.inner[key]
    }

    /// Returns a mutable reference to the value with the equivalent key to the given one, if it
    /// exists.
    ///
    /// # Panics
    ///
    /// This method panics if an equivalent key to the key `key` does not exist in the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    ///     (Key::new(3), 4_f64),
    /// ]);
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(1)), &mut 2_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(2)), &mut 3_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(3)), &mut 4_f64);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     *map.get_mut_unchecked(&Key::new(1)) *= 2_f64;
    ///     *map.get_mut_unchecked(&Key::new(2)) *= 2_f64;
    ///     *map.get_mut_unchecked(&Key::new(3)) *= 2_f64;
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(1)), &mut 4_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(2)), &mut 6_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(3)), &mut 8_f64);
    /// }
    /// ```
    #[track_caller]
    pub fn get_mut_unchecked<Q>(&mut self, key: &Q) -> &mut T
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        &mut self.inner[key]
    }

    /// Attempts to get mutable references to multiple values at once in the hash map.
    ///
    /// This method returns an array of length `N` supplied by the query. For each value returned,
    /// this method returns `Some(&mut value_i)`, where `value_i` is the ith value corresponding
    /// to the ith key provided in the `ks` argument. If a key does not exist in the map, the
    /// corresponding value returned will be `None` in the array.
    ///
    /// If `N < self.len::<T>()`, at least one of the keys will not exist in the map, so this is
    /// the same situation as the query having keys that do not exist in the map.
    ///
    /// # Panics
    ///
    /// This method panics if any of the keys are overlapping to preserve soundness.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert(Key::new(1), String::from("Dark Souls"));
    /// het_map.insert(Key::new(2), String::from("Dark Souls II"));
    /// het_map.insert(Key::new(3), String::from("Dark Souls III"));
    /// het_map.insert(Key::new(4), String::from("Bloodborne"));
    /// het_map.insert(Key::new(5), String::from("Sekiro: Shadows Die Twice"));
    /// het_map.insert(Key::new(6), String::from("Elden Ring"));
    /// het_map.insert(Key::new(7), String::from("Nioh"));
    ///
    /// let expected = [
    ///     &mut String::from("Bloodborne"),
    ///     &mut String::from("Elden Ring"),
    ///     &mut String::from("Nioh"),
    /// ];
    /// let map = het_map.get_map_mut::<String>().unwrap();
    /// let result = map.get_disjoint_mut([&Key::new(4), &Key::new(0), &Key::new(7), &Key::new(6)]);
    ///
    /// assert_eq!(result[0], Some(&mut String::from("Bloodborne")));
    /// assert_eq!(result[1], None);
    /// assert_eq!(result[2], Some(&mut String::from("Nioh")));
    /// assert_eq!(result[3], Some(&mut String::from("Elden Ring")));
    /// ```
    #[track_caller]
    pub fn get_disjoint_mut<Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&'_ mut T>; N]
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.get_disjoint_mut(ks)
    }

    /// Inserts a new entry into the hash map.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent key already exists in the hash map, this method replaces the old value
    ///   with the new value in the map, and returns the old value as `Some(old_value)`.
    /// * If the entry with the equivalent key does not exist in the map, it is inserted into the
    ///   hash map and the method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<f64>();
    /// {
    ///     let map = het_map.get_map::<f64>().unwrap();
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), None);
    ///     assert_eq!(map.get(&Key::new(2)), None);
    ///     assert_eq!(map.get(&Key::new(3)), None);
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.insert(Key::new(1), 2_f64), None);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 1);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2)), None);
    ///     assert_eq!(map.get(&Key::new(3)), None);
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.insert(Key::new(2), 3_f64), None);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 2);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3)), None);
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.insert(Key::new(3), 4_f64), None);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map::<f64>().unwrap();
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// ```
    pub fn insert(&mut self, key: Key<K, T>, value: T) -> Option<T>
    where
        K: hash::Hash + Eq,
    {
        self.inner.insert(key, value)
    }

    /// Removes an entry with an equivalent key to the given key from the hash map.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent key already exists in the hash map, this method removes the entry
    ///   from the hash map and returns `Some(value)`, where `value` is the value corresponding
    ///   to the equivalent key to `key`.
    /// * If the entry with the equivalent key does not exist in the map, this method does nothing
    ///   and returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<f64>();
    /// {
    ///     let map = het_map.get_map::<f64>().unwrap();
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), None);
    ///     assert_eq!(map.get(&Key::new(2)), None);
    ///     assert_eq!(map.get(&Key::new(3)), None);
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     map.insert(Key::new(1), 2_f64);
    ///     map.insert(Key::new(2), 3_f64);
    ///     map.insert(Key::new(3), 4_f64);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove(&Key::new(1)), Some(2_f64));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 2);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), None);
    ///     assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove(&Key::new(2)), Some(3_f64));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 1);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), None);
    ///     assert_eq!(map.get(&Key::new(2)), None);
    ///     assert_eq!(map.get(&Key::new(3)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove(&Key::new(3)), Some(4_f64));
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), None);
    ///     assert_eq!(map.get(&Key::new(2)), None);
    ///     assert_eq!(map.get(&Key::new(3)), None);
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// ```
    pub fn remove<Q>(&mut self, key: &Q) -> Option<T>
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.swap_remove(key)
    }

    /// Removes an entry with an equivalent key to the given key from the hash map.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent key already exists in the hash map, this method removes the entry
    ///   from the hash map and returns `Some((eq_key, value))`, where `eq_key` is the equivalent
    ///   to key to `key` for the entry, and `value` is the value corresponding to the equivalent
    ///   key to `key`.
    /// * If the entry with the equivalent key does not exist in the map, this method does nothing
    ///   and returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<f64>();
    /// {
    ///     let map = het_map.get_map::<f64>().unwrap();
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get(&Key::new(1)), None);
    ///     assert_eq!(map.get(&Key::new(2)), None);
    ///     assert_eq!(map.get(&Key::new(3)), None);
    ///     assert_eq!(map.get(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     map.insert(Key::new(1), 2_f64);
    ///     map.insert(Key::new(2), 3_f64);
    ///     map.insert(Key::new(3), 4_f64);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1)), Some((&Key::new(1), &2_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(2)), Some((&Key::new(2), &3_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(3)), Some((&Key::new(3), &4_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove_entry(&Key::new(1)), Some((Key::new(1), 2_f64)));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 2);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(2)), Some((&Key::new(2), &3_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(3)), Some((&Key::new(3), &4_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove_entry(&Key::new(2)), Some((Key::new(2), 3_f64)));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 1);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(2)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(3)), Some((&Key::new(3), &4_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(4)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove_entry(&Key::new(3)), Some((Key::new(3), 4_f64)));
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(2)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(3)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(4)), None);
    /// }
    /// ```
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(Key<K, T>, T)>
    where
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.swap_remove_entry(key)
    }

    /// Returns an iterator over the entries of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let entries = vec![
    ///     (Key::new(1), String::from("foo")),
    ///     (Key::new(2), String::from("bar")),
    ///     (Key::new(3), String::from("baz")),
    /// ];
    /// het_map.extend(entries.clone());
    /// let map = het_map.get_map::<String>().unwrap();
    ///
    /// let mut iter = map.iter().peekable();
    /// while iter.peek().is_some() {
    ///     let entry = iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap();
    ///     assert!(entries.contains(&entry));
    /// }
    ///
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> Iter<'_, K, T> {
        Iter::new(self.inner.iter())
    }

    /// Returns a mutable iterator over the entries of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let entries = vec![
    ///     (Key::new(1), String::from("foo")),
    ///     (Key::new(2), String::from("bar")),
    ///     (Key::new(3), String::from("baz")),
    /// ];
    /// het_map.extend(entries.clone());
    /// let map = het_map.get_map_mut::<String>().unwrap();
    ///
    /// let mut iter = map.iter_mut().peekable();
    /// while iter.peek().is_some() {
    ///     let entry = iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap();
    ///     assert!(entries.contains(&entry));
    /// }
    ///
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, T> {
        IterMut::new(self.inner.iter_mut())
    }

    /// Returns an iterator over the keys of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let keys = vec![Key::new(1), Key::new(2), Key::new(3)];
    /// let values = vec![String::from("foo"), String::from("bar"), String::from("baz")];
    /// het_map.extend(keys.iter().cloned().zip(values.iter().cloned()));
    /// let map = het_map.get_map::<String>().unwrap();
    ///
    /// let mut iter = map.keys().peekable();
    /// while iter.peek().is_some() {
    ///     let key = iter.next().cloned().unwrap();
    ///     assert!(keys.contains(&key));
    /// }
    ///
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn keys(&self) -> Keys<'_, K, T> {
        Keys::new(self.inner.keys())
    }

    /// Returns an iterator over the values of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let keys = vec![Key::new(1), Key::new(2), Key::new(3)];
    /// let values = vec![String::from("foo"), String::from("bar"), String::from("baz")];
    /// het_map.extend(keys.iter().cloned().zip(values.iter().cloned()));
    /// let map = het_map.get_map::<String>().unwrap();
    ///
    /// let mut iter = map.values().peekable();
    /// while iter.peek().is_some() {
    ///     let value = iter.next().cloned().unwrap();
    ///     assert!(values.contains(&value));
    /// }
    ///
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn values(&self) -> Values<'_, K, T> {
        Values::new(self.inner.values())
    }

    /// Returns a mutable iterator over the values of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let keys = vec![Key::new(1), Key::new(2), Key::new(3)];
    /// let values = vec![String::from("foo"), String::from("bar"), String::from("baz")];
    /// het_map.extend(keys.iter().cloned().zip(values.iter().cloned()));
    /// let map = het_map.get_map_mut::<String>().unwrap();
    ///
    /// let mut iter = map.values_mut().peekable();
    /// while iter.peek().is_some() {
    ///     let value = iter.next().cloned().unwrap();
    ///     assert!(values.contains(&value));
    /// }
    ///
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, T> {
        ValuesMut::new(self.inner.values_mut())
    }

    /// Returns a draining iterator over the entries of the hash map.
    ///
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed
    /// elements. The returned iterator keeps a mutable borrow on the hash map to optimize its
    /// implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), String::from("Dark Souls")),
    ///     (Key::new(2), String::from("Dark Souls II")),
    ///     (Key::new(3), String::from("Dark Souls III")),
    ///     (Key::new(4), String::from("Bloodborne")),
    ///     (Key::new(5), String::from("Sekiro: Shadows Die Twice")),
    ///     (Key::new(6), String::from("Elden Ring")),
    ///     (Key::new(7), String::from("Nioh")),
    /// ]);
    /// assert_eq!(het_map.len::<String>(), Some(7));
    /// {
    ///     let map = het_map.get_map_mut::<String>().unwrap();
    ///
    ///     assert_eq!(map.len(), 7);
    ///
    ///     map.drain();
    ///
    ///     assert_eq!(map.len(), 0);
    /// }
    /// assert_eq!(het_map.len::<String>(), Some(0));
    /// ```
    pub fn drain(&mut self) -> Drain<'_, K, T> {
        Drain::new(self.inner.drain(..))
    }

    /// Creates an iterator which uses a closure to determine if an element should be removed.
    ///
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed
    /// elements. The returned iterator keeps a mutable borrow on the hash map to optimize its
    /// implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HomogeneousHashMap, HeterogeneousHashMap};
    /// # use std::collections::HashMap;
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1), String::from("Dark Souls")),
    ///     (Key::new(2), String::from("Dark Souls II")),
    ///     (Key::new(3), String::from("Dark Souls III")),
    ///     (Key::new(4), String::from("Bloodborne")),
    ///     (Key::new(5), String::from("Sekiro: Shadows Die Twice")),
    ///     (Key::new(6), String::from("Elden Ring")),
    ///     (Key::new(7), String::from("Nioh")),
    /// ]);
    /// let expected = HomogeneousHashMap::from([
    ///     (Key::new(1), String::from("Dark Souls")),
    ///     (Key::new(2), String::from("Dark Souls II")),
    ///     (Key::new(3), String::from("Dark Souls III")),
    /// ]);
    /// let result = {
    ///     let map = het_map.get_map_mut::<String>().unwrap();
    ///     let extracted: HomogeneousHashMap<usize, String> = map.extract_if(|k, v| v.contains("Dark Souls")).collect();
    ///     extracted
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn extract_if<F>(&mut self, keep: F) -> ExtractIf<'_, K, T, F>
    where
        F: FnMut(&Key<K, T>, &mut T) -> bool,
    {
        ExtractIf::new(self.inner.extract_if(.., keep))
    }

    /// Removes all the entries from the hash map.
    ///
    /// After calling this method, the collection will be empty. This method does not change the
    /// allocated capacity of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(0), String::from("foo")),
    ///     (Key::new(1), String::from("bar")),
    ///     (Key::new(2), String::from("baz")),
    /// ]);
    /// let map = het_map.get_map_mut::<String>().unwrap();
    ///
    /// assert!(!map.is_empty());
    /// assert_eq!(map.len(), 3);
    /// let old_capacity = map.capacity();
    ///
    /// map.clear();
    ///
    /// assert!(map.is_empty());
    /// assert_eq!(map.len(), 0);
    /// assert_eq!(map.capacity(), old_capacity);
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Retains only the key-value pairs specified by the predicate.
    ///
    /// This method removes all entries `e` for which `keep(&e)` returns `false`. This method
    /// visits each element exactly once, keeping only those entries `e` for which `keep(&e)`
    /// returns `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// fn len_is_odd(k: &Key<usize, &str>, v: &mut &str) -> bool { v.len() % 2 != 0 }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(0), "foo"),
    ///     (Key::new(1), "bar"),
    ///     (Key::new(2), "baz"),
    ///     (Key::new(3), "quux"),
    ///     (Key::new(4), "quuz"),
    ///     (Key::new(5), "corge"),
    ///     (Key::new(6), "grault"),
    ///     (Key::new(7), "garply"),
    ///     (Key::new(8), "waldo"),
    /// ]);
    /// let mut expected_het_map = HeterogeneousHashMap::new();
    /// expected_het_map.extend(Vec::from([
    ///     (Key::new(0), "foo"),
    ///     (Key::new(1), "bar"),
    ///     (Key::new(2), "baz"),
    ///     (Key::new(5), "corge"),
    ///     (Key::new(8), "waldo"),
    /// ]));
    /// {
    ///     let map = het_map.get_map_mut::<&str>().unwrap();
    ///     map.retain(len_is_odd);
    /// }
    /// let expected = expected_het_map.get_map::<&str>();
    /// let result = het_map.get_map::<&str>();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&Key<K, T>, &mut T) -> bool,
    {
        self.inner.retain(keep)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given hash map.
    ///
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling this method, the capacity will be greater than or equal to
    /// `self.len() + additional` if it returns. This method does nothing if the collection
    /// capacity is already sufficient. This method preserves the contents even if a panic occurs.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    ///
    /// * If the capacity of the hash map overflows.
    /// * If the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let map = het_map.get_or_insert_map_mut::<i32>();
    /// map.extend([
    ///     (Key::new(0), 1_i32),
    ///     (Key::new(1), 2_i32),
    ///     (Key::new(2), 3_i32),
    ///     (Key::new(3), 4_i32),
    ///     (Key::new(4), 5_i32),
    ///     (Key::new(5), 6_i32),
    /// ]);
    /// map.reserve(10);
    ///
    /// assert!(map.capacity() >= map.len() + 10);
    ///
    /// let old_capacity = map.capacity();
    /// map.extend([
    ///     (Key::new(6), 7_i32),
    ///     (Key::new(7), 8_i32),
    ///     (Key::new(8), 9_i32),
    ///     (Key::new(9), 10_i32),
    /// ]);
    ///
    /// assert_eq!(map.capacity(), old_capacity);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given hash map.
    ///
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling this method, the capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`. This method does nothing if the collection
    /// capacity is already sufficient. This method preserves the contents even if an error occurs.
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity overflows, or the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let map = het_map.get_or_insert_map_mut();
    /// map.extend([
    ///     (Key::new(0), 1_i32),
    ///     (Key::new(1), 2_i32),
    ///     (Key::new(2), 3_i32),
    ///     (Key::new(3), 4_i32),
    ///     (Key::new(4), 5_i32),
    ///     (Key::new(5), 6_i32),
    /// ]);
    /// let result = map.try_reserve(10);
    ///
    /// assert!(result.is_ok());
    /// assert!(map.capacity() >= map.len() + 10);
    ///
    /// let old_capacity = map.capacity();
    /// map.extend([
    ///     (Key::new(6), 7_i32),
    ///     (Key::new(7), 8_i32),
    ///     (Key::new(8), 9_i32),
    ///     (Key::new(9), 10_i32),
    /// ]);
    ///
    /// assert_eq!(map.capacity(), old_capacity);
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        fn from_opaque_try_reserve_error(error: opaque::error::TryReserveError) -> TryReserveError {
            TryReserveError::from(match error.kind() {
                opaque::error::TryReserveErrorKind::CapacityOverflow => TryReserveErrorKind::CapacityOverflow,
                opaque::error::TryReserveErrorKind::AllocError { layout } => TryReserveErrorKind::AllocError { layout },
            })
        }

        self.inner.try_reserve(additional).map_err(from_opaque_try_reserve_error)
    }

    /// Shrinks the capacity of the hash map as much as possible.
    ///
    /// The resulting hash map might still have some excess capacity, just as is the case for
    /// [`with_capacity`]. This depends on the resize policy for the internal structure.
    ///
    /// [`with_capacity`]: HomogeneousHashMap::with_capacity
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let map = het_map.get_or_insert_with_capacity_map_mut::<i32>(10);
    /// map.extend([(Key::new(0), 1_i32), (Key::new(1), 2_i32), (Key::new(2), 3_i32)]);
    ///
    /// assert!(map.capacity() >= 10);
    ///
    /// map.shrink_to_fit();
    ///
    /// assert!(map.capacity() >= 3);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks the capacity of the hash map to a lower bound.
    ///
    /// The resulting hash map might still have some excess capacity, just as is the case for
    /// [`with_capacity`]. This depends on the resize policy for the internal structure.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied capacity `min_capacity`. In particular, after calling this method,
    /// the capacity of `self` satisfies
    ///
    /// ```text
    /// self.capacity() >= max(self.len(), min_capacity).
    /// ```
    ///
    /// If the current capacity of the hash map is less than the lower bound, the method does
    /// nothing.
    ///
    /// [`with_capacity`]: TypeProjectedIndexMap::with_capacity
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let map = het_map.get_or_insert_with_capacity_map_mut::<i32>(10);
    /// map.extend([(Key::new(0), 1_i32), (Key::new(1), 2_i32), (Key::new(2), 3_i32)]);
    ///
    /// assert!(map.capacity() >= 10);
    ///
    /// map.shrink_to(4);
    ///
    /// assert!(map.capacity() >= 4);
    ///
    /// map.shrink_to(0);
    ///
    /// assert!(map.capacity() >= 3);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

impl<K, T, S> PartialEq for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq,
    T: any::Any + PartialEq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.inner, &other.inner)
    }
}

impl<K, T, S> Eq for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq,
    T: any::Any + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
}

impl<K, T, S> fmt::Debug for HomogeneousHashMap<K, T, S>
where
    K: any::Any + fmt::Debug,
    T: any::Any + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<K, T, S> ops::Index<&Key<K, T>> for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    type Output = T;

    fn index(&self, key: &Key<K, T>) -> &Self::Output {
        &self.inner[key]
    }
}

impl<K, T, S> Extend<(Key<K, T>, T)> for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (Key<K, T>, T)>,
    {
        self.inner.extend(iterable);
    }
}

impl<'a, K, T, S> Extend<(&'a Key<K, T>, &'a T)> for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq + Copy,
    T: any::Any + Copy,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (&'a Key<K, T>, &'a T)>,
    {
        self.inner.extend(iterable);
    }
}

impl<K, T, S> FromIterator<(Key<K, T>, T)> for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (Key<K, T>, T)>,
    {
        let mut map = HomogeneousHashMap::with_hasher(S::default());
        map.extend(iterable);

        map
    }
}

impl<K, T, S, const N: usize> From<[(Key<K, T>, T); N]> for HomogeneousHashMap<K, T, S>
where
    K: any::Any + hash::Hash + Eq,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from(array: [(Key<K, T>, T); N]) -> Self {
        HomogeneousHashMap::from_iter(array)
    }
}

impl<K, T, S> Clone for HomogeneousHashMap<K, T, S>
where
    K: any::Any + Clone,
    T: any::Any + Clone,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone();

        HomogeneousHashMap::from_inner(cloned_inner)
    }
}

impl<K, T, S> Default for HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn default() -> Self {
        HomogeneousHashMap::with_hasher(S::default())
    }
}

/// An iterator over the metadata of the types stored in a heterogeneous hash map.
///
/// Type metadata iterators are returned by the [`HeterogeneousHashMap::metadata_iter`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::HeterogeneousHashMap;
/// # use core::any::TypeId;
/// #
/// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
/// het_map.insert_type::<u16>();
/// het_map.insert_type::<i32>();
/// het_map.insert_type::<f64>();
///
/// let type_ids = [TypeId::of::<u16>(), TypeId::of::<i32>(), TypeId::of::<f64>()];
/// let mut iter = het_map.metadata_iter();
///
/// assert!(type_ids.contains(iter.next().unwrap().0));
/// assert!(type_ids.contains(iter.next().unwrap().0));
/// assert!(type_ids.contains(iter.next().unwrap().0));
/// assert!(iter.next().is_none());
/// ```
pub struct TypeMetadataIter<'a> {
    iter: hash_map::Iter<'a, any::TypeId, TypeMetadata>,
}

impl<'a> TypeMetadataIter<'a> {
    /// Constructs a new type metadata iterator.
    #[inline]
    const fn new(iter: hash_map::Iter<'a, any::TypeId, TypeMetadata>) -> Self {
        Self { iter }
    }
}

impl<'a> Iterator for TypeMetadataIter<'a> {
    type Item = (&'a any::TypeId, &'a TypeMetadata);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> ExactSizeIterator for TypeMetadataIter<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a> iter::FusedIterator for TypeMetadataIter<'a> {}

impl<'a> Clone for TypeMetadataIter<'a> {
    fn clone(&self) -> Self {
        Self::new(self.iter.clone())
    }
}

impl<'a> Default for TypeMetadataIter<'a> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

/// The metadata for a data type stored inside a [`HeterogeneousHashMap`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeMetadata {
    type_id: any::TypeId,
    type_name: &'static str,
    size: usize,
    alignment: usize,
}

impl TypeMetadata {
    /// Computes the metadata associated with the type `T` stored inside a heterogeneous hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.type_id(), TypeId::of::<String>());
    /// assert_eq!(metadata.type_name(), any::type_name::<String>());
    /// assert_eq!(metadata.size(), mem::size_of::<String>());
    /// assert_eq!(metadata.alignment(), mem::align_of::<String>());
    /// ```
    pub fn of<T>() -> Self
    where
        T: any::Any,
    {
        TypeMetadata {
            type_id: any::TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
            size: mem::size_of::<T>(),
            alignment: mem::align_of::<T>(),
        }
    }

    /// Returns the type identifier from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.type_id(), TypeId::of::<String>());
    /// ```
    #[inline]
    pub const fn type_id(&self) -> any::TypeId {
        self.type_id
    }

    /// Returns the type name from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.type_name(), any::type_name::<String>());
    /// ```
    #[inline]
    pub const fn type_name(&self) -> &str {
        self.type_name
    }

    /// Returns the size of a type from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.size(), mem::size_of::<String>());
    /// ```
    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Returns the memory alignment of a type from the type metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::TypeMetadata;
    /// # use core::any;
    /// # use core::any::TypeId;
    /// # use core::mem;
    /// #
    /// let metadata = TypeMetadata::of::<String>();
    ///
    /// assert_eq!(metadata.alignment(), mem::align_of::<String>());
    /// ```
    #[inline]
    pub const fn alignment(&self) -> usize {
        self.alignment
    }
}

/// A heterogeneous hash map that can store values of more than one data type.
///
/// This collection is a useful container for situations where you want to store and retrieve
/// differently-typed values under shared keys, without writing custom boilerplate or using
/// complex enum workarounds. Perfect for systems where logic depends on the value's type.
/// Since a heterogeneous hash map can store any [`Any`] type, it is an extremely flexible building
/// block for many such situations.
///
/// # Data Structure Organization
///
/// This hash map is a hierarchical two level hash map with both a type key, and value key,
/// given by the [`Key<K, T>`] data type. In particular, this hashmap is different from an ordinary
/// [`HashMap`] in that it can store vales of multiple unrelated data types in the same collection.
/// This type accomplishes this by keying against the [`TypeId`] of each type stored in it.
/// This has one caveat that the heterogeneous hash map can only store types with the [`Any`]
/// trait, but this is not a big deal since virtually every type that can be expressed in Rust's
/// type system implements [`Any`].
///
/// Every type in a heterogeneous hash map gets dedicated per-type storage separate from the other
/// data types. This way, mutating elements of one type is guaranteed not to affect any other
/// types in the hash map.
///
/// The heterogeneous hash map also stores type metadata for each data type stored in the
/// collection at any given time by keeping a [`TypeMetadata`] entry inside the map for each type
/// stored in the map. Rust is a systems language, so it does not have runtime reflection baked
/// in, so the user of the hash map has to have some way of knowing a priori what types are stored
/// in the map. The [`TypeMetadata`] feature provides limited type introspection abilities to
/// facilitate this task.
///
/// # Limitations
///
/// While the heterogeneous hash map can store multiple value types, it uses exactly one hash
/// builder type (typically the default one from the standard library), and one backing memory
/// allocator (typically the global one).
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
/// # use core::f64;
/// #
/// // Some weird mathematical constants.
/// let champernowne_constant: f64 = 0.123456789101112_f64;
/// let liouville_number: f64 = 0.110001000000000000000001_f64;
/// let feigenbaum_delta: f64 = 4.669_201_609_102_990_f64;
/// let khinchin_constant: f64 = 2.685_452_001_f64;
/// let gelfond_schneider_constant: f64 = f64::powf(2_f64, f64::sqrt(2_f64));
///
/// let mut het_map = HeterogeneousHashMap::new();
/// // Inserting new types.
/// assert!(het_map.insert_type::<i32>());
/// assert!(het_map.insert_type::<u16>());
/// // Inserting entries for existing types.
/// het_map.extend([
///     (Key::new(1), 2_i32),
///     (Key::new(2), 3_i32),
///     (Key::new(3), 5_i32),
/// ]);
/// het_map.extend((0..=15).map(|i| (Key::new(i), 1_u16 << i)));
///
/// // A heterogeneous hash map inserts a new type automatically when inserting elements of a new type.
/// het_map.extend::<_, f64>([
///     (Key::new(2),  champernowne_constant),
///     (Key::new(3),  liouville_number),
///     (Key::new(5),  feigenbaum_delta),
///     (Key::new(7),  khinchin_constant),
///     (Key::new(11), gelfond_schneider_constant),
/// ]);
///
/// // Checking the contents of the heterogeneous hash map before manipulation.
/// {
///     assert_eq!(het_map.len_types(), 3);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), Some(5));
///
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3)), Some(&5_i32));
///
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2)),  Some(&champernowne_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3)),  Some(&liouville_number));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5)),  Some(&feigenbaum_delta));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7)),  Some(&khinchin_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11)), Some(&gelfond_schneider_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13)), None);
/// }
///
/// // Removing an element of a specific type from a heterogeneous hash map.
/// {
///     assert_eq!(het_map.remove::<f64, _>(&Key::new(5)), Some(feigenbaum_delta));
///
///     assert_eq!(het_map.len_types(), 3);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), Some(4));
///
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3)), Some(&5_i32));
///
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2)),  Some(&champernowne_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3)),  Some(&liouville_number));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7)),  Some(&khinchin_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11)), Some(&gelfond_schneider_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13)), None);
/// }
///
/// // Removing an entire type from a heterogeneous hash map.
/// {
///     // The map currently contains the `f64` type.
///     assert!(het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.remove_type::<f64>(), Some(4));
///
///     // The map no longer contains the `f64` type.
///     assert!(!het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.len_types(), 2);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), None);
///
///     // The `i32` entries remain untouched.
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3)), Some(&5_i32));
///
///     // The `u16` entries remain untouched.
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     // Every `f64` entry was removed from the map.
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11)), None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13)), None);
/// }
///
/// // Inserting one value of a new data type into a heterogeneous hash map.
/// {
///     assert!(!het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.insert(Key::new(13), f64::consts::PI), None);
///
///     // The map now contains the `f64` type again.
///     assert!(het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.len_types(), 3);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), Some(1));
///
///     // The `i32` entries remain untouched.
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3)), Some(&5_i32));
///
///     // The `u16` entries remain untouched.
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     // The previous `f64` entries prior to the call to `remove_type` no longer exist, but the
///     // newly inserted one does.
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11)), None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13)), Some(&f64::consts::PI));
/// }
///
/// // Clearing the entire map.
/// {
///     assert!(het_map.contains_type::<i32>());
///     assert!(het_map.contains_type::<u16>());
///     assert!(het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.len_types(), 3);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), Some(1));
///
///     het_map.clear();
///
///     // The heterogeneous hash map no longer contains any types.
///     assert!(!het_map.contains_type::<i32>());
///     assert!(!het_map.contains_type::<u16>());
///     assert!(!het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.len_types(), 0);
///     assert_eq!(het_map.len::<i32>(), None);
///     assert_eq!(het_map.len::<u16>(), None);
///     assert_eq!(het_map.len::<f64>(), None);
///
///     // Every value of every type `i32` is gone.
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2)), None);
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3)), None);
///
///     // Every value of every type `u16` is gone.
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), None);
///     }
///
///     // Every value of type `f64` is gone.
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11)), None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13)), None);
/// }
///
/// assert!(het_map.is_empty_types());
/// ```
#[cfg(feature = "std")]
pub struct HeterogeneousHashMap<K, S = hash::RandomState>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    map: hash_map::HashMap<any::TypeId, opaque::index_map::TypeErasedIndexMap, S>,
    registry: hash_map::HashMap<any::TypeId, TypeMetadata, S>,
    build_hasher: S,
    _marker: marker::PhantomData<K>,
}

#[cfg(not(feature = "std"))]
pub struct HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    map: hash_map::HashMap<any::TypeId, opaque::index_map::TypeErasedIndexMap, S>,
    registry: hash_map::HashMap<any::TypeId, TypeMetadata, S>,
    build_hasher: S,
    _marker: marker::PhantomData<K>,
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Constructs a new empty heterogeneous hash map with the custom hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// # use std::hash::RandomState;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize, RandomState> = HeterogeneousHashMap::with_hasher(RandomState::new());
    ///
    /// assert!(het_map.is_empty_types());
    /// assert_eq!(het_map.len_types(), 0);
    /// assert_eq!(het_map.capacity_types(), 0);
    ///
    /// het_map.insert_type::<i8>();
    ///
    /// assert!(!het_map.is_empty_types());
    /// assert_eq!(het_map.len_types(), 1);
    /// assert!(het_map.capacity_types() > 0);
    /// ```
    pub fn with_hasher(build_hasher: S) -> Self {
        Self {
            map: hash_map::HashMap::with_hasher(build_hasher.clone()),
            registry: hash_map::HashMap::with_hasher(build_hasher.clone()),
            build_hasher,
            _marker: marker::PhantomData,
        }
    }

    /// Constructs a new empty heterogeneous hash map with at least the specified capacity and
    /// using the custom hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// # use std::hash::RandomState;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize, RandomState> = HeterogeneousHashMap::with_capacity_and_hasher(3, RandomState::new());
    ///
    /// assert!(het_map.is_empty_types());
    /// assert_eq!(het_map.len_types(), 0);
    /// assert!(het_map.capacity_types() >= 3);
    ///
    /// let old_capacity = het_map.capacity_types();
    /// het_map.insert_type::<i8>();
    /// het_map.insert_type::<i16>();
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(!het_map.is_empty_types());
    /// assert_eq!(het_map.len_types(), 3);
    /// assert_eq!(het_map.capacity_types(), old_capacity);
    /// ```
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: S) -> Self {
        Self {
            map: hash_map::HashMap::with_capacity_and_hasher(capacity, build_hasher.clone()),
            registry: hash_map::HashMap::with_capacity_and_hasher(capacity, build_hasher.clone()),
            build_hasher,
            _marker: marker::PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<K> HeterogeneousHashMap<K, hash::RandomState>
where
    K: any::Any,
{
    /// Constructs a new empty heterogeneous hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(het_map.is_empty_types());
    /// assert_eq!(het_map.len_types(), 0);
    /// assert_eq!(het_map.capacity_types(), 0);
    ///
    /// het_map.insert_type::<i8>();
    ///
    /// assert!(!het_map.is_empty_types());
    /// assert_eq!(het_map.len_types(), 1);
    /// assert!(het_map.capacity_types() > 0);
    /// ```
    pub fn new() -> Self {
        Self::with_hasher(hash::RandomState::new())
    }

    /// Constructs a new empty heterogeneous hash map with a minimum type capacity of `capacity`.
    ///
    /// The **type capacity** of a heterogeneous hash map is the minimum number of types the map
    /// can store without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::with_capacity(3);
    ///
    /// assert!(het_map.is_empty_types());
    /// assert_eq!(het_map.len_map(), 0);
    /// assert!(het_map.capacity_types() >= 3);
    /// let old_capacity = het_map.capacity_types();
    ///
    /// het_map.insert_type::<i8>();
    /// het_map.insert_type::<i16>();
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(het_map.capacity_types() >= old_capacity);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, hash::RandomState::new())
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Inserts a new type into the heterogeneous hash map.
    ///
    /// This method registers the type in the heterogeneous hash map, but does not allocate memory
    /// for values of that type. A subsequent value insertion will trigger a memory allocation to
    /// occur.
    ///
    /// This method returns `true` if the type `T` does not exist in the map. This method returns
    /// `false` if the type `T` already exists in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.capacity::<i32>(), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.capacity::<i32>(), Some(0));
    ///
    /// het_map.insert::<i32>(Key::new(0), 1_i32);
    ///
    /// assert!(het_map.capacity::<i32>() >= Some(0));
    /// ```
    pub fn insert_type<T>(&mut self) -> bool
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        if self.map.contains_key(&type_id) {
            return false;
        }

        let type_metadata = TypeMetadata::of::<T>();
        let map = opaque::index_map::TypeErasedIndexMap::new::<Key<K, T>, T>();

        self.registry.insert(type_id, type_metadata);
        self.map.insert(type_id, map);

        debug_assert_eq!(self.registry.len(), self.map.len());

        true
    }

    /// Inserts a new type into the heterogeneous hash map with a given minimum type capacity.
    ///
    /// This method registers the type in the heterogeneous hash map, and allocates memory for at
    /// least `capacity` entries for values of that type.
    ///
    /// This method returns `true` if the type `T` does not exist in the map. This method returns
    /// `false` if the type `T` already exists in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.capacity::<i32>(), None);
    ///
    /// het_map.insert_type_with_capacity::<i32>(3);
    ///
    /// assert!(het_map.capacity::<i32>() >= Some(3));
    /// let old_capacity = het_map.capacity::<i32>();
    ///
    /// het_map.insert::<i32>(Key::new(0), 1_i32);
    /// het_map.insert::<i32>(Key::new(1), 2_i32);
    /// het_map.insert::<i32>(Key::new(1), 3_i32);
    ///
    /// assert_eq!(het_map.capacity::<i32>(), old_capacity);
    /// ```
    pub fn insert_type_with_capacity<T>(&mut self, capacity: usize) -> bool
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        if self.map.contains_key(&type_id) {
            return false;
        }

        let type_metadata = TypeMetadata::of::<T>();
        let map = opaque::index_map::TypeErasedIndexMap::with_capacity::<Key<K, T>, T>(capacity);

        self.registry.insert(type_id, type_metadata);
        self.map.insert(type_id, map);

        debug_assert_eq!(self.registry.len(), self.map.len());

        true
    }

    /// Determines whether a heterogeneous hash map contains the given type.
    ///
    /// This method returns `true` if the type `T` exists in the map. This method returns `false`
    /// if the type `T` does not exist in the map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_type::<i32>());
    /// assert!(!het_map.contains_type::<u64>());
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(!het_map.contains_type::<u64>());
    ///
    /// het_map.insert_type::<u64>();
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<u64>());
    ///
    /// het_map.remove_type::<i32>();
    ///
    /// assert!(!het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<u64>());
    ///
    /// het_map.remove_type::<u64>();
    ///
    /// assert!(!het_map.contains_type::<i32>());
    /// assert!(!het_map.contains_type::<u64>());
    /// ```
    pub fn contains_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();

        self.map.contains_key(&type_id)
    }

    /// Returns a reference to the hash map containing all values of a given type from the
    /// heterogeneous hash map.
    ///
    /// # Panics
    ///
    /// This method panics if the given type `T` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    ///
    /// let map = het_map.get_map_unchecked::<i32>();
    ///
    /// assert_eq!(map.len(), 0);
    /// ```
    pub fn get_map_unchecked<T>(&self) -> &HomogeneousHashMap<K, T, S>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        let map = self.map[&type_id].as_proj::<Key<K, T>, T, S, alloc::Global>();

        HomogeneousHashMap::from_inner_ref(map)
    }

    /// Returns a mutable reference to the hash map containing all values of a given type from the
    /// heterogeneous hash map.
    ///
    /// # Panics
    ///
    /// This method panics if the given type `T` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::HeterogeneousHashMap;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    ///
    /// let map = het_map.get_map_mut_unchecked::<i32>();
    ///
    /// assert_eq!(map.len(), 0);
    /// ```
    pub fn get_map_mut_unchecked<T>(&mut self) -> &mut HomogeneousHashMap<K, T, S>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        let map = self.map
            .get_mut(&type_id)
            .unwrap()
            .as_proj_mut::<Key<K, T>, T, S, alloc::Global>();

        HomogeneousHashMap::from_inner_ref_mut(map)
    }

    /// Returns a reference to the hash map containing all values of a given type from the
    /// heterogeneous hash map.
    ///
    /// If `T` exists in the heterogeneous hash map, this method returns `Some(&map)`, where
    /// `map` is the hash map containing all values of type `T`. If `T` does not exist in the
    /// heterogeneous map, this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u32>();
    /// het_map.extend([
    ///     (Key::new(0), 2_i32),
    ///     (Key::new(1), 3_i32),
    ///     (Key::new(2), 5_i32),
    /// ]);
    ///
    /// let maybe_map1 = het_map.get_map::<i32>();
    /// let maybe_map2 = het_map.get_map::<u32>();
    /// let maybe_map3 = het_map.get_map::<&str>();
    ///
    /// assert!(maybe_map1.is_some());
    /// assert!(maybe_map2.is_some());
    /// assert!(maybe_map3.is_none());
    ///
    /// let map1 = maybe_map1.unwrap();
    /// let map2 = maybe_map2.unwrap();
    ///
    /// assert_eq!(map1.len(), 3);
    /// assert_eq!(map2.len(), 0);
    /// ```
    pub fn get_map<T>(&self) -> Option<&HomogeneousHashMap<K, T, S>>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            return None;
        }

        let map = self.map
            .get(&type_id)
            .map(|m| m.as_proj::<Key<K, T>, T, S, alloc::Global>())?;

        Some(HomogeneousHashMap::from_inner_ref(map))
    }

    /// Returns a mutable reference to the hash map containing all values of a given type from the
    /// heterogeneous hash map.
    ///
    /// If `T` exists in the heterogeneous hash map, this method returns `Some(&mut map)`, where
    /// `map` is the hash map containing all values of type `T`. If `T` does not exist in the
    /// heterogeneous map, this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u32>();
    /// het_map.extend([
    ///     (Key::new(0), 2_i32),
    ///     (Key::new(1), 3_i32),
    ///     (Key::new(2), 5_i32),
    /// ]);
    /// {
    ///     let maybe_map1 = het_map.get_map_mut::<i32>();
    ///     assert!(maybe_map1.is_some());
    /// }
    /// {
    ///     let maybe_map2 = het_map.get_map_mut::<u32>();
    ///     assert!(maybe_map2.is_some());
    /// }
    /// {
    ///     let maybe_map3 = het_map.get_map_mut::<&str>();
    ///     assert!(maybe_map3.is_none());
    /// }
    /// {
    ///     let map1 = het_map.get_map_mut::<i32>().unwrap();
    ///     assert_eq!(map1.len(), 3);
    /// }
    /// {
    ///     let map2 = het_map.get_map_mut::<u32>().unwrap();
    ///     assert_eq!(map2.len(), 0);
    /// }
    /// ```
    pub fn get_map_mut<T>(&mut self) -> Option<&mut HomogeneousHashMap<K, T, S>>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            return None;
        }

        let map = self.map
            .get_mut(&type_id)
            .map(|m| m.as_proj_mut::<Key<K, T>, T, S, alloc::Global>())?;

        Some(HomogeneousHashMap::from_inner_ref_mut(map))
    }

    /// Returns a mutable reference to the hash map containing all values of a given type from the
    /// heterogeneous hash map.
    ///
    /// If the type `T` does not exist in the heterogeneous hash map, this method inserts the type
    /// `T` into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u32>();
    /// het_map.extend([
    ///     (Key::new(0), 2_i32),
    ///     (Key::new(1), 3_i32),
    ///     (Key::new(2), 5_i32),
    /// ]);
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<u32>());
    /// assert!(!het_map.contains_type::<f64>());
    /// {
    ///     let map1 = het_map.get_or_insert_map_mut::<i32>();
    ///     assert_eq!(map1.len(), 3);
    /// }
    /// {
    ///     let map2 = het_map.get_or_insert_map_mut::<u32>();
    ///     assert_eq!(map2.len(), 0);
    /// }
    /// {
    ///     let map3 = het_map.get_or_insert_map_mut::<f64>();
    ///     assert_eq!(map3.len(), 0);
    /// }
    ///
    /// assert!(het_map.contains_type::<f64>());
    /// ```
    pub fn get_or_insert_map_mut<T>(&mut self) -> &mut HomogeneousHashMap<K, T, S>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            self.insert_type::<T>();
        }

        debug_assert_eq!(self.registry.len(), self.map.len());

        self.get_map_mut::<T>().unwrap()
    }

    /// Returns a mutable reference to the hash map containing all values of a given type from the
    /// heterogeneous hash map.
    ///
    /// If the type `T` does not exist in the heterogeneous hash map, this method inserts the type
    /// `T` into the map with a minimum capacity specified by `capacity`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u32>();
    /// het_map.extend([
    ///     (Key::new(0), 2_i32),
    ///     (Key::new(1), 3_i32),
    ///     (Key::new(2), 5_i32),
    /// ]);
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<u32>());
    /// assert!(!het_map.contains_type::<f64>());
    /// {
    ///     let map1 = het_map.get_or_insert_with_capacity_map_mut::<i32>(10);
    ///     assert_eq!(map1.len(), 3);
    /// }
    /// {
    ///     let map2 = het_map.get_or_insert_with_capacity_map_mut::<u32>(10);
    ///     assert_eq!(map2.len(), 0);
    /// }
    /// {
    ///     let map3 = het_map.get_or_insert_with_capacity_map_mut::<f64>(10);
    ///     assert_eq!(map3.len(), 0);
    ///     assert!(map3.capacity() >= 10);
    /// }
    ///
    /// assert!(het_map.contains_type::<f64>());
    /// ```
    pub fn get_or_insert_with_capacity_map_mut<T>(&mut self, capacity: usize) -> &mut HomogeneousHashMap<K, T, S>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            self.insert_type_with_capacity::<T>(capacity);
        }

        debug_assert_eq!(self.registry.len(), self.map.len());

        self.get_map_mut::<T>().unwrap()
    }

    /// Removes a type from a heterogeneous hash map.
    ///
    /// This method behaves as follows:
    ///
    /// * If the given type `T` exists in the heterogeneous hash map, this method removes every
    ///   value of type `T` from the map, deallocates memory allocated for values of type `T`, and
    ///   returns `Some(count)`, when `count` is the number of values of type `T` that were stored
    ///   in the map. This method returns `Some(0)` even when the type `T` exists in the map, but
    ///   no values of type `T` do.
    /// * If the given type `T` does not exist in the heterogeneous hash map, this method returns
    ///   `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_type::<i32>());
    /// assert!(!het_map.contains_type::<f64>());
    /// assert_eq!(het_map.len_types(), 0);
    /// assert_eq!(het_map.len_map(), 0);
    ///
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<f64>();
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<f64>());
    /// assert_eq!(het_map.len_types(), 2);
    /// assert_eq!(het_map.len_map(), 0);
    ///
    /// het_map.extend([
    ///     (Key::new(0), 1_f64),
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    /// ]);
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<i32>());
    /// assert_eq!(het_map.len_types(), 2);
    /// assert_eq!(het_map.len_map(), 3);
    ///
    /// assert_eq!(het_map.len::<i32>(), Some(0));
    /// assert_eq!(het_map.len::<f64>(), Some(3));
    ///
    /// // Removing the type `i32` from the heterogeneous hash map.
    /// assert_eq!(het_map.remove_type::<i32>(), Some(0));
    ///
    /// // Verifying that the type `i32` no longer exists in the heterogeneous hash map.
    /// {
    ///     assert!(!het_map.contains_type::<i32>());
    ///     assert!(het_map.contains_type::<f64>());
    ///     assert_eq!(het_map.len_types(), 1);
    ///     assert_eq!(het_map.len_map(), 3);
    ///
    ///     assert_eq!(het_map.len::<i32>(), None);
    ///     assert_eq!(het_map.len::<f64>(), Some(3));
    /// }
    ///
    /// // Removing the type `f64` from the heterogeneous hash map.
    /// assert_eq!(het_map.remove_type::<f64>(), Some(3));
    ///
    /// // Verifying that the type `f64` no longer exists in the heterogeneous hash map.
    /// {
    ///     assert!(!het_map.contains_type::<i32>());
    ///     assert!(!het_map.contains_type::<f64>());
    ///     assert_eq!(het_map.len_types(), 0);
    ///     assert_eq!(het_map.len_map(), 0);
    ///
    ///     assert_eq!(het_map.len::<i32>(), None);
    ///     assert_eq!(het_map.len::<f64>(), None);
    /// }
    ///
    /// assert!(het_map.is_empty_types());
    /// ```
    pub fn remove_type<T>(&mut self) -> Option<usize>
    where
        T: any::Any,
    {
        let removed_count = {
            let map = self.get_map_mut::<T>()?;
            let _removed_count = map.len();
            map.clear();
            _removed_count
        };

        let type_id = any::TypeId::of::<T>();
        self.map.remove(&type_id);
        self.registry.remove(&type_id);

        debug_assert_eq!(self.registry.len(), self.map.len());

        Some(removed_count)
    }

    /// Removes a type from a heterogeneous hash map and returns them as a hash map.
    ///
    /// This method behaves as follows:
    ///
    /// * If the given type `T` exists in the heterogeneous hash map, this method removes every
    ///   value of type `T` from the map, deallocates all memory for values of type `T`, and
    ///   returns `Some(map)`, where `map` is a hash map containing all the removed entries of
    ///   type `T` from the heterogeneous hash map.
    /// * If the given type `T` does not exist in the heterogeneous hash map, this method returns
    ///   `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_type::<i32>());
    /// assert!(!het_map.contains_type::<f64>());
    /// assert_eq!(het_map.len_types(), 0);
    /// assert_eq!(het_map.len_map(), 0);
    ///
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<f64>();
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<f64>());
    /// assert_eq!(het_map.len_types(), 2);
    /// assert_eq!(het_map.len_map(), 0);
    ///
    /// het_map.extend([
    ///     (Key::new(0), 1_f64),
    ///     (Key::new(1), 2_f64),
    ///     (Key::new(2), 3_f64),
    /// ]);
    ///
    /// assert!(het_map.contains_type::<i32>());
    /// assert!(het_map.contains_type::<i32>());
    /// assert_eq!(het_map.len_types(), 2);
    /// assert_eq!(het_map.len_map(), 3);
    ///
    /// assert_eq!(het_map.len::<i32>(), Some(0));
    /// assert_eq!(het_map.len::<f64>(), Some(3));
    ///
    /// // Taking the type `f64` from the heterogeneous hash map.
    /// {
    ///     // Taking all entries of type `f64` from the heterogeneous hash map.
    ///     let result = het_map.take_type::<f64>();
    ///
    ///     assert!(result.is_some());
    ///
    ///     let map = result.unwrap();
    ///
    ///     // Checking that every entry of type `f64` was returned.
    ///     assert_eq!(map.len(), 3);
    ///     assert_eq!(map.get(&Key::new(0)), Some(&1_f64));
    ///     assert_eq!(map.get(&Key::new(1)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2)), Some(&3_f64));
    /// }
    ///
    ///  // Verifying that the type `f64` no longer exists in the heterogeneous hash map.
    /// {
    ///     assert!(het_map.contains_type::<i32>());
    ///     assert!(!het_map.contains_type::<f64>());
    ///
    ///     assert_eq!(het_map.len_types(), 1);
    ///     assert_eq!(het_map.len_map(), 0);
    ///
    ///     assert_eq!(het_map.len::<i32>(), Some(0));
    ///     assert_eq!(het_map.len::<f64>(), None);
    /// }
    ///
    /// // Taking the type `i32` from the heterogeneous hash map.
    /// {
    ///     // Taking all entries of type `i32` from the heterogeneous hash map.
    ///     let result = het_map.take_type::<i32>();
    ///
    ///     // This method returns `Some(..)` as long as type `i32` exists in the map, even if
    ///     // no values of type `i32` do.
    ///     assert!(result.is_some());
    ///
    ///     let map = result.unwrap();
    ///
    ///     // Checking that every entry of type `f64` was returned.
    ///     assert_eq!(map.len(), 0);
    /// }
    ///
    /// // Verifying that the type `i32` no longer exists in the heterogeneous hash map.
    /// {
    ///     assert!(!het_map.contains_type::<i32>());
    ///     assert!(!het_map.contains_type::<f64>());
    ///
    ///     assert_eq!(het_map.len_types(), 0);
    ///     assert_eq!(het_map.len_map(), 0);
    ///
    ///     assert_eq!(het_map.len::<i32>(), None);
    ///     assert_eq!(het_map.len::<f64>(), None);
    /// }
    ///
    /// assert!(het_map.is_empty_types());
    /// ```
    pub fn take_type<T>(&mut self) -> Option<HomogeneousHashMap<K, T, S>>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();
        let removed_map = self.map.remove(&type_id)?;
        self.registry.remove(&type_id);

        debug_assert_eq!(self.registry.len(), self.map.len());

        Some(HomogeneousHashMap::from_inner(removed_map.into_proj::<Key<K, T>, T, S, alloc::Global>()))
    }

    /// Removes all types and all values for each type from the heterogeneous hash map.
    ///
    /// This method removes every value of every type stored in the map, removes every type
    /// from the map, and deallocates memory that was allocated for each type removed from the
    /// map. The map retains its type capacity after calling this method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<f64>();
    /// het_map.extend([(Key::new(1), 2_f64), (Key::new(2), 3_f64), (Key::new(3), 4_f64)]);
    ///
    /// assert_eq!(het_map.len_types(), 2);
    /// assert_eq!(het_map.len_map(), 3);
    /// assert_eq!(het_map.len::<i32>(), Some(0));
    /// assert_eq!(het_map.len::<f64>(), Some(3));
    /// assert!(het_map.capacity::<i32>().is_some());
    /// assert!(het_map.capacity::<f64>().is_some());
    ///
    /// let old_capacity_types = het_map.capacity_types();
    /// het_map.clear();
    ///
    /// assert_eq!(het_map.len_types(), 0);
    /// assert_eq!(het_map.len_map(), 0);
    /// assert_eq!(het_map.len::<i32>(), None);
    /// assert_eq!(het_map.len::<f64>(), None);
    /// assert!(het_map.capacity::<i32>().is_none());
    /// assert!(het_map.capacity::<f64>().is_none());
    ///
    /// assert_eq!(het_map.capacity_types(), old_capacity_types);
    /// ```
    pub fn clear(&mut self) {
        let type_ids = Vec::from_iter(self.map.keys().cloned());
        for type_id in type_ids.iter() {
            let _ = self.map.remove(type_id);
            let _ = self.registry.remove(type_id);
        }

        debug_assert_eq!(self.registry.len(), 0);
        debug_assert_eq!(self.map.len(), 0);
    }
}

impl<S, K> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Computes the total number of values across all types in the heterogeneous hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let values1 = [
    ///     (Key::new(1), 3_i32),
    ///     (Key::new(2), 5_i32),
    ///     (Key::new(3), 7_i32),
    /// ];
    /// let values2 = [
    ///     (Key::new(1), 2_u64),
    /// ];
    /// let values3 = [
    ///     (Key::new(1), String::from("foo")),
    ///     (Key::new(2), String::from("bar")),
    ///     (Key::new(3), String::from("baz")),
    ///     (Key::new(4), String::from("quux")),
    ///     (Key::new(5), String::from("garply")),
    /// ];
    ///
    /// assert_eq!(het_map.len_map(), 0);
    ///
    /// het_map.extend(values1.iter().cloned());
    ///
    /// assert_eq!(het_map.len_map(), values1.len());
    ///
    /// het_map.extend(values2.iter().cloned());
    ///
    /// assert_eq!(het_map.len_map(), values1.len() + values2.len());
    ///
    /// het_map.extend(values3.iter().cloned());
    ///
    /// assert_eq!(het_map.len_map(), values1.len() + values2.len() + values3.len());
    /// ```
    pub fn len_map(&self) -> usize {
        let mut len = 0;
        for map in self.map.values() {
            len += map.len();
        }

        len
    }

    /// Returns a reference to the heterogeneous hash map's hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any;
    /// # use core::hash;
    /// # use std::hash::RandomState;
    /// #
    /// #[derive(Clone)]
    /// struct WrappingBuildHasher<S> {
    ///     inner: S,
    /// }
    ///
    /// impl<S> WrappingBuildHasher<S>
    /// where
    ///     S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    ///     S::Hasher: any::Any + hash::Hasher + Send + Sync,
    /// {
    ///     fn new(inner: S) -> Self {
    ///         Self { inner }
    ///     }
    /// }
    ///
    /// impl<S> hash::BuildHasher for WrappingBuildHasher<S>
    /// where
    ///     S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    ///     S::Hasher: any::Any + hash::Hasher + Send + Sync,
    /// {
    ///     type Hasher = S::Hasher;
    ///
    ///     fn build_hasher(&self) -> S::Hasher {
    ///         self.inner.build_hasher()
    ///     }
    /// }
    ///
    /// let mut het_map: HeterogeneousHashMap<usize, WrappingBuildHasher<RandomState>> = HeterogeneousHashMap::with_hasher(
    ///     WrappingBuildHasher::new(RandomState::new()),
    /// );
    /// let build_hasher: &WrappingBuildHasher<RandomState> = het_map.hasher();
    /// ```
    #[inline]
    pub const fn hasher(&self) -> &S {
        &self.build_hasher
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns the number of types stored in the heterogeneous hash map.
    ///
    /// # Examples
    ///
    /// Getting the type length of an empty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.len_types(), 0);
    /// ```
    ///
    /// Getting the type length of a nonempty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any::Any;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u64>();
    /// het_map.insert_type::<f64>();
    /// het_map.insert_type::<Box<dyn Any>>();
    ///
    /// assert_eq!(het_map.len_types(), 4);
    /// ```
    ///
    /// Getting the type length of a nonempty heterogeneous hash map with at last one value.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any::Any;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.len_types(), 0);
    ///
    /// het_map.insert::<i32>(Key::new(0), 1_i32);
    ///
    /// assert_eq!(het_map.len_types(), 1);
    /// ```
    #[inline]
    pub fn len_types(&self) -> usize {
        self.map.len()
    }

    /// Returns the maximum number of types the heterogeneous hash map can store without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any::Any;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::with_capacity(10);
    ///
    /// assert!(het_map.capacity_types() >= 10);
    /// let old_capacity = het_map.capacity_types();
    ///
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u64>();
    /// het_map.insert_type::<f64>();
    /// het_map.insert_type::<Box<dyn Any>>();
    ///
    /// assert_eq!(het_map.capacity_types(), old_capacity);
    /// ```
    #[inline]
    pub fn capacity_types(&self) -> usize {
        self.map.capacity()
    }

    /// Determines whether the heterogeneous hash map is empty.
    ///
    /// A heterogeneous hash map is **empty** if it contains no types. Equivalently, a
    /// heterogeneous hash map is empty if and only if its length is zero.
    ///
    /// This method returns `true` if the heterogeneous hash map contains no types. This method
    /// returns `false` if the heterogeneous hash map contains at least one type.
    ///
    /// A heterogeneous hash map that contains no types also contains no values, That is a
    /// heterogeneous hash map that contains at least one value of a given type must also have
    /// at least one type.
    ///
    /// # Examples
    ///
    /// Calling this method on an empty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(het_map.is_empty_types());
    /// ```
    ///
    /// Calling this method on a nonempty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any::Any;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<u64>();
    /// het_map.insert_type::<f64>();
    /// het_map.insert_type::<Box<dyn Any>>();
    ///
    /// assert_eq!(het_map.len_types(), 4);
    /// ```
    #[inline]
    pub fn is_empty_types(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns the capacity the heterogeneous hash map has for the given type.
    ///
    /// The **capacity** for a given type is the maximum number of values of the given type the
    /// heterogeneous hash map can store without reallocating memory.
    ///
    /// This method returns `Some(capacity)` if the given type `T` exists in the map. This method
    /// returns `None` if the given type `T` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type_with_capacity::<f64>(10);
    ///
    /// assert_eq!(het_map.capacity::<i32>(), Some(0));
    /// assert!(het_map.capacity::<f64>() >= Some(10));
    /// assert_eq!(het_map.capacity::<String>(), None);
    /// ```
    pub fn capacity<T>(&self) -> Option<usize>
    where
        T: any::Any,
    {
        let map = self.get_map::<T>()?;

        Some(map.capacity())
    }

    /// Returns the number of elements the heterogeneous hash map has for the given type.
    ///
    /// The **length** for a given type is the number of values of the given type the
    /// heterogeneous hash map currently stores.
    ///
    /// This method returns `Some(length)` if the given type `T` exists in the map. This method
    /// returns `None` if the given type `T` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::f64;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type_with_capacity::<f64>(10);
    /// het_map.extend([
    ///     (Key::new(0), f64::consts::SQRT_2),
    ///     (Key::new(1), f64::consts::PI),
    ///     (Key::new(2), f64::consts::E),
    /// ]);
    ///
    /// assert_eq!(het_map.len::<i32>(),    Some(0));
    /// assert_eq!(het_map.len::<f64>(),    Some(3));
    /// assert_eq!(het_map.len::<String>(), None);
    /// ```
    pub fn len<T>(&self) -> Option<usize>
    where
        T: any::Any,
    {
        let map = self.get_map::<T>()?;

        Some(map.len())
    }

    /// Determines whether the heterogeneous hash map contains no values of the given type.
    ///
    /// This method behaves as follows:
    ///
    /// * If the heterogeneous hash map contains the type `T`, and contains at least one value
    ///   of type `T`, this method returns `Some(true)`.
    /// * If the heterogeneous hash map contains the type `T`, but contains no values of type
    ///   `T`, this method returns `Some(false)`.
    /// * If the heterogeneous hash map does not contain the type `T`, this method returns `None`.
    ///
    /// In particular, this method returns and [`Option<bool>`] instead of a regular [`bool`] to
    /// distinguish between the last two cases above.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::f64;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type_with_capacity::<f64>(10);
    /// het_map.extend([
    ///     (Key::new(0), f64::consts::SQRT_2),
    ///     (Key::new(1), f64::consts::PI),
    ///     (Key::new(2), f64::consts::E),
    /// ]);
    ///
    /// assert_eq!(het_map.is_empty::<i32>(),    Some(true));
    /// assert_eq!(het_map.is_empty::<f64>(),    Some(false));
    /// assert_eq!(het_map.is_empty::<String>(), None);
    /// ```
    pub fn is_empty<T>(&self) -> Option<bool>
    where
        T: any::Any,
    {
        let map = self.get_map::<T>()?;

        Some(map.is_empty())
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns the type metadata for a given type in the heterogeneous hash map.
    ///
    /// This method returns `Some(metadata)` where `metadata` is the metadata describing the given
    /// type in the map, if the type exists in the map. This method returns `None` if the given
    /// type does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(het_map.get_metadata::<i32>().is_none());
    /// assert!(het_map.get_metadata::<String>().is_none());
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(het_map.get_metadata::<i32>().is_some());
    /// assert!(het_map.get_metadata::<String>().is_none());
    ///
    /// het_map.insert_type::<String>();
    ///
    /// assert!(het_map.get_metadata::<i32>().is_some());
    /// assert!(het_map.get_metadata::<String>().is_some());
    /// ```
    pub fn get_metadata<T>(&self) -> Option<TypeMetadata>
    where
        T: any::Any,
    {
        let type_id = any::TypeId::of::<T>();

        self.registry.get(&type_id).cloned()
    }

    /// Returns the type metadata for a given type with a given [`TypeId`] in the heterogeneous
    /// hash map.
    ///
    /// This method returns `Some(metadata)` where `metadata` is the metadata describing the given
    /// type in the map, if the type exists in the map. This method returns `None` if the given
    /// type does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any::TypeId;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(het_map.get_metadata_by_id(TypeId::of::<i32>()).is_none());
    /// assert!(het_map.get_metadata_by_id(TypeId::of::<String>()).is_none());
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(het_map.get_metadata_by_id(TypeId::of::<i32>()).is_some());
    /// assert!(het_map.get_metadata_by_id(TypeId::of::<String>()).is_none());
    ///
    /// het_map.insert_type::<String>();
    ///
    /// assert!(het_map.get_metadata_by_id(TypeId::of::<i32>()).is_some());
    /// assert!(het_map.get_metadata_by_id(TypeId::of::<String>()).is_some());
    /// ```
    pub fn get_metadata_by_id(&self, type_id: any::TypeId) -> Option<TypeMetadata> {
        self.registry.get(&type_id).cloned()
    }

    /// Returns an iterator over the metadata of all the types stored in the heterogeneous hash
    /// map.
    ///
    /// # Examples
    ///
    /// Iterating over the metadata of an empty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// let mut iter = het_map.metadata_iter();
    ///
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// Iterating over the metadata of a nonempty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// # use core::any::Any;
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// het_map.insert_type::<String>();
    /// het_map.insert_type::<Box<dyn Any>>();
    /// let mut iter = het_map.metadata_iter();
    ///
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn metadata_iter(&self) -> TypeMetadataIter<'_> {
        TypeMetadataIter::new(self.registry.iter())
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Determines whether a heterogeneous hash map contains a value of the given type with the given key.
    ///
    /// This method returns `true` if the key-value pair with the given type and the given key
    /// exists in the map. This method returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// Querying an empty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(0)));
    /// ```
    ///
    /// Querying a nonempty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(0)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(0)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    ///
    /// het_map.insert(Key::new(0), i32::MAX);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(0)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    /// ```
    pub fn contains_key<T, Q>(&self, key: &Q) -> bool
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let type_id = any::TypeId::of::<T>();
        match self.map.get(&type_id) {
            Some(opaque_map) => {
                let proj_map = opaque_map.as_proj::<Key<K, T>, T, S, alloc::Global>();
                proj_map.contains_key(key)
            }
            None => false,
        }
    }

    /// Returns a reference to the value of the given type with the given key, if the type
    /// and key exist in the heterogeneous hash map.
    ///
    /// This method returns `Some(&value)`, where `value` has the given type, and an equivalent key
    /// to the key `key` exists in the map corresponding to the value `value`. This method returns
    /// `None` if the value with the given type and the given key `key` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    ///
    /// het_map.insert(Key::new(0), i32::MAX);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), Some(&i32::MAX));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    /// ```
    pub fn get<T, Q>(&self, key: &Q) -> Option<&T>
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map::<T>()?;

        map.get(key)
    }

    /// Returns references to the key and value of the given type with the given key, if the type
    /// and key exist in the heterogeneous hash map.
    ///
    /// This method returns `Some((&eq_key, &value))`, where `value` has the given type, and an
    /// equivalent key `eq_key` to the key `key` exists in the map corresponding to the value
    /// `value`. This method returns `None` if the key-value pair with the given value type and an
    /// equivalent key to the given key `key` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1)), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1)), None);
    ///
    /// het_map.insert(Key::new(0), i32::MAX);
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(0)), Some((&Key::new(0), &i32::MAX)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1)), None);
    /// ```
    pub fn get_key_value<T, Q>(&self, key: &Q) -> Option<(&Key<K, T>, &T)>
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map::<T>()?;

        map.get_key_value(key)
    }

    /// Returns a mutable reference to the value of the given type with the given key, if the type
    /// and key exist in the heterogeneous hash map.
    ///
    /// This method returns `Some(&value)`, where `value` has the given type, and an equivalent key
    /// to the key `key` exists in the map corresponding to the value `value`. This method returns
    /// `None` if the value with the given type and the given key `key` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    ///
    /// het_map.insert(Key::new(0), i32::MAX);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), Some(&i32::MAX));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    /// ```
    pub fn get_mut<T, Q>(&mut self, key: &Q) -> Option<&mut T>
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_mut::<T>()?;

        map.get_mut(key)
    }

    /// Attempts to get mutable references to multiple values at once in the heterogeneous hash
    /// map.
    ///
    /// This method returns an array of length `N` supplied by the query. For each value returned,
    /// this method returns `Some(&mut value_i)`, where `value_i` is the ith value corresponding
    /// to the ith key provided in the `ks` argument. If a key does not exist in the map, the
    /// corresponding value returned will be `None` in the array.
    ///
    /// If `N < self.len::<T>()`, at least one of the keys will not exist in the map, so this is
    /// the same situation as the query having keys that do not exist in the map.
    ///
    /// # Panics
    ///
    /// This method panics if any of the keys are overlapping to preserve soundness.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert(Key::new(1), String::from("Dark Souls"));
    /// het_map.insert(Key::new(2), String::from("Dark Souls II"));
    /// het_map.insert(Key::new(3), String::from("Dark Souls III"));
    /// het_map.insert(Key::new(4), String::from("Bloodborne"));
    /// het_map.insert(Key::new(5), String::from("Sekiro: Shadows Die Twice"));
    /// het_map.insert(Key::new(6), String::from("Elden Ring"));
    /// het_map.insert(Key::new(7), String::from("Nioh"));
    ///
    /// let expected = [
    ///     &mut String::from("Bloodborne"),
    ///     &mut String::from("Elden Ring"),
    ///     &mut String::from("Nioh"),
    /// ];
    /// let result = het_map.get_disjoint_mut::<String, _, 4>([&Key::new(4), &Key::new(0), &Key::new(7), &Key::new(6)]);
    ///
    /// assert_eq!(result[0], Some(&mut String::from("Bloodborne")));
    /// assert_eq!(result[1], None);
    /// assert_eq!(result[2], Some(&mut String::from("Nioh")));
    /// assert_eq!(result[3], Some(&mut String::from("Elden Ring")));
    /// ```
    #[inline]
    #[track_caller]
    pub fn get_disjoint_mut<T, Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&'_ mut T>; N]
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_mut::<T>().unwrap();

        map.get_disjoint_mut(ks)
    }

    /// Inserts a new key-value pair of a given type with a given key into the heterogeneous hash
    /// map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    ///
    /// het_map.insert(Key::new(1), 5_i32);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(1)));
    /// ```
    pub fn insert<T>(&mut self, key: Key<K, T>, value: T) -> Option<T>
    where
        K: hash::Hash + Eq,
        T: any::Any,
    {
        let map = self.get_or_insert_map_mut::<T>();

        map.insert(key, value)
    }

    /// Removes an entry from a heterogeneous hash map of the given type with the given key, if it
    /// exists.
    ///
    /// This method returns `Some(value)`, where `value` is the value associated with the
    /// equivalent key to the key `key` if it exists in the map. If an equivalent key to `key`
    /// does not exist in the map, this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(2)));
    ///
    /// het_map.insert(Key::new(1), 3_i32);
    /// het_map.insert(Key::new(2), 5_i32);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(1)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2)));
    ///
    /// assert_eq!(het_map.remove::<i32, _>(&Key::new(1)), Some(3_i32));
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2)));
    /// ```
    pub fn remove<T, Q>(&mut self, key: &Q) -> Option<T>
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_mut::<T>()?;

        map.remove(key)
    }

    /// Removes an entry from a heterogeneous hash map of the given type with the given key, if it
    /// exists.
    ///
    /// This method returns `Some((eq_key, value))`, where `eq_key` is equivalent key to the key
    /// `key` in the map, and `value` is the value associated with `eq_key`, if an equivalent key
    /// to the key `key` exists in the map. If an equivalent key to `key` does not exist in the map,
    /// this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(2)));
    ///
    /// het_map.insert(Key::new(1), 3_i32);
    /// het_map.insert(Key::new(2), 5_i32);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(1)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2)));
    ///
    /// assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1)), Some((Key::new(1), 3_i32)));
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2)));
    /// ```
    pub fn remove_entry<T, Q>(&mut self, key: &Q) -> Option<(Key<K, T>, T)>
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_mut::<T>()?;

        map.remove_entry(key)
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns a reference to the value of the given type with the given key, if it exists
    /// in the heterogeneous hash map.
    ///
    /// # Panics
    ///
    /// This method panics if the given type `T` does not exist in the map, or the type exists,
    /// but a key equivalent to the key `key` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert(Key::new(1), i32::MAX);
    ///
    /// assert_eq!(het_map.get_unchecked::<i32, _>(&Key::new(1)), &i32::MAX);
    /// ```
    #[track_caller]
    pub fn get_unchecked<T, Q>(&self, key: &Q) -> &T
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_unchecked::<T>();

        map.get_unchecked(key)
    }

    /// Returns a mutable reference to the value of the given type with the given key, if it exists
    /// in the heterogeneous hash map.
    ///
    /// # Panics
    ///
    /// This method panics if the given type `T` does not exist in the map, or the type exists,
    /// but a key equivalent to the key `key` does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert(Key::new(1), i32::MAX);
    ///
    /// assert_eq!(het_map.get_mut_unchecked::<i32, _>(&Key::new(1)), &i32::MAX);
    /// ```
    #[track_caller]
    pub fn get_mut_unchecked<T, Q>(&mut self, key: &Q) -> &mut T
    where
        T: any::Any,
        Key<K, T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_mut_unchecked::<T>();

        map.get_mut_unchecked(key)
    }
}

impl<K, S> HeterogeneousHashMap<K, S>
where
    K: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Inserts one or more values of the given type to the heterogeneous hash map from an
    /// iterable.
    ///
    /// If the given type does not exist in the map when this method is called, this method
    /// inserts the type into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(2)), None);
    ///
    /// assert_eq!(het_map.get::<String, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(3)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(5)), None);
    ///
    /// het_map.extend([
    ///     (Key::new(0), 1_i32),
    ///     (Key::new(1), 2_i32),
    ///     (Key::new(2), 3_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), Some(&1_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), Some(&2_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(2)), Some(&3_i32));
    ///
    /// assert_eq!(het_map.get::<String, _>(&Key::new(0)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(3)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(5)), None);
    ///
    /// het_map.extend([
    ///     (Key::new(0), String::from("foo")),
    ///     (Key::new(3), String::from("bar")),
    ///     (Key::new(5), String::from("baz")),
    /// ]);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0)), Some(&1_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1)), Some(&2_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(2)), Some(&3_i32));
    ///
    /// assert_eq!(het_map.get::<String, _>(&Key::new(0)), Some(&String::from("foo")));
    /// assert_eq!(het_map.get::<String, _>(&Key::new(3)), Some(&String::from("bar")));
    /// assert_eq!(het_map.get::<String, _>(&Key::new(5)), Some(&String::from("baz")));
    /// ```
    #[inline]
    pub fn extend<I, T>(&mut self, iterable: I)
    where
        T: any::Any,
        I: IntoIterator<Item = (Key<K, T>, T)>,
    {
        let map = self.get_or_insert_map_mut::<T>();

        map.extend(iterable)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct TypeEntry<'a> {
    type_id: &'a any::TypeId,
    metadata: &'a TypeMetadata,
    length: usize,
}

impl<S> fmt::Debug for HeterogeneousHashMap<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries: Vec<TypeEntry> = self.map.iter().map(|(type_id, map)| {
            let metadata = self.registry.get(type_id)
                .expect("Every stored type must have registered metadata");

            TypeEntry {
                type_id,
                metadata,
                length: map.len(),
            }
        }).collect();

        f.debug_struct("HeterogeneousHashMap")
            .field("entries", &entries)
            .finish()
    }
}

#[cfg(test)]
mod test_internals {
    use std::boxed::Box;
    use std::string::String;
    use super::*;

    fn match_map_type_ids(het_map: &HeterogeneousHashMap<hash::RandomState>) {
        for (type_id, map) in het_map.map.iter() {
            assert_eq!(map.value_type_id(), *type_id);
        }
    }

    fn match_registry_type_ids(het_map: &HeterogeneousHashMap<hash::RandomState>) {
        for (type_id, metadata) in het_map.registry.iter() {
            assert_eq!(metadata.type_id, *type_id);
        }
    }

    #[test]
    fn test_map_type_ids() {
        let mut het_map = HeterogeneousHashMap::new();

        match_map_type_ids(&het_map);
        het_map.insert_type::<()>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<bool>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<i8>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<i16>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<i32>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<i64>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<isize>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<u8>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<u16>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<u32>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<u64>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<usize>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<f32>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<f64>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<char>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<String>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<&str>();
        match_map_type_ids(&het_map);
        het_map.insert_type::<Box<dyn any::Any>>();
        match_map_type_ids(&het_map);
    }

    #[test]
    fn test_registry_type_ids() {
        let mut het_map = HeterogeneousHashMap::new();

        match_registry_type_ids(&het_map);
        het_map.insert_type::<()>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<bool>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<i8>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<i16>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<i32>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<i64>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<isize>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<u8>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<u16>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<u32>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<u64>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<usize>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<f32>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<f64>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<char>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<String>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<&str>();
        match_registry_type_ids(&het_map);
        het_map.insert_type::<Box<dyn any::Any>>();
        match_registry_type_ids(&het_map);
    }

    #[test]
    fn test_remove_type_map_type_ids() {
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

        match_map_type_ids(&het_map);
        het_map.remove_type::<()>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<bool>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<i8>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<i16>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<i32>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<i64>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<isize>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<u8>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<u16>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<u32>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<u64>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<usize>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<f32>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<f64>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<char>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<String>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<&str>();
        match_map_type_ids(&het_map);
        het_map.remove_type::<Box<dyn any::Any>>();
        match_map_type_ids(&het_map);
    }

    #[test]
    fn test_remove_type_registry_type_ids() {
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

        match_registry_type_ids(&het_map);
        het_map.remove_type::<()>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<bool>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<i8>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<i16>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<i32>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<i64>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<isize>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<u8>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<u16>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<u32>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<u64>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<usize>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<f32>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<f64>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<char>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<String>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<&str>();
        match_registry_type_ids(&het_map);
        het_map.remove_type::<Box<dyn any::Any>>();
        match_registry_type_ids(&het_map);
    }
}
