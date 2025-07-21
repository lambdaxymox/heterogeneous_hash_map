use crate::error::{
    TryReserveErrorKind,
    TryReserveError,
};
use crate::iterator::{
    Drain,
    ExtractIf,
    Iter,
    IterMut,
    Keys,
    Values,
    ValuesMut,
    IntoIter,
    IntoKeys,
    IntoValues,
};
use crate::key::Key;

use core::any;
use core::borrow::Borrow;
use core::fmt;
use core::ops;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

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
///     (Key::new(0_usize), 2_u32),
///     (Key::new(1_usize), 3_u32),
///     (Key::new(2_usize), 5_u32),
///     (Key::new(3_usize), 7_u32),
///     (Key::new(4_usize), 11_u32),
///     (Key::new(5_usize), 13_u32),
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
    pub(crate) const fn from_inner(inner: opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>) -> Self {
        Self { inner }
    }

    /// Constructs a new hash map.
    #[inline]
    pub(crate) const fn from_inner_ref(map: &opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>) -> &Self {
        unsafe { &*(map as *const opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S> as *const Self) }
    }

    /// Constructs a new hash map.
    #[inline]
    pub(crate) const fn from_inner_ref_mut(map: &mut opaque::index_map::TypeProjectedIndexMap<Key<K, T>, T, S>) -> &mut Self {
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
    /// map.insert(Key::new(0_usize), 1_i32);
    /// map.insert(Key::new(1_usize), 2_i32);
    /// map.insert(Key::new(2_usize), 3_i32);
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
    /// map.insert(Key::new(0_usize), 1_i32);
    /// map.insert(Key::new(1_usize), 2_i32);
    /// map.insert(Key::new(2_usize), 3_i32);
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
    ///     (Key::new(0_usize), String::from("foo")),
    ///     (Key::new(1_usize), String::from("bar")),
    ///     (Key::new(2_usize), String::from("baz")),
    /// ]);
    /// let map = het_map.get_map_mut::<String>().unwrap();
    ///
    /// assert_eq!(map.capacity(), old_capacity);
    ///
    /// map.insert(Key::new(3_usize), String::from("quux"));
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
    ///     (Key::new(0_usize), String::from("foo")),
    ///     (Key::new(1_usize), String::from("bar")),
    ///     (Key::new(2_usize), String::from("baz")),
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
    ///     (Key::new(0_usize), String::from("foo")),
    ///     (Key::new(1_usize), String::from("bar")),
    ///     (Key::new(2_usize), String::from("baz")),
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
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
    ///     (Key::new(3_usize), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert!(map.contains_key(&Key::new(1_usize)));
    /// assert!(map.contains_key(&Key::new(2_usize)));
    /// assert!(map.contains_key(&Key::new(3_usize)));
    /// assert!(!map.contains_key(&Key::new(4_usize)));
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
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
    ///     (Key::new(3_usize), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    /// assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
    /// assert_eq!(map.get(&Key::new(3_usize)), Some(&4_f64));
    /// assert_eq!(map.get(&Key::new(4_usize)), None);
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
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
    ///     (Key::new(3_usize), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert_eq!(map.get_key_value(&Key::new(1_usize)), Some((&Key::new(1_usize), &2_f64)));
    /// assert_eq!(map.get_key_value(&Key::new(2_usize)), Some((&Key::new(2_usize), &3_f64)));
    /// assert_eq!(map.get_key_value(&Key::new(3_usize)), Some((&Key::new(3_usize), &4_f64)));
    /// assert_eq!(map.get_key_value(&Key::new(4_usize)), None);
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
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
    ///     (Key::new(3_usize), 4_f64),
    /// ]);
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut(&Key::new(1_usize)), Some(&mut 2_f64));
    ///     assert_eq!(map.get_mut(&Key::new(2_usize)), Some(&mut 3_f64));
    ///     assert_eq!(map.get_mut(&Key::new(3_usize)), Some(&mut 4_f64));
    ///     assert_eq!(map.get_mut(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     map.get_mut(&Key::new(1_usize)).map(|v| *v *= 2_f64);
    ///     map.get_mut(&Key::new(2_usize)).map(|v| *v *= 2_f64);
    ///     map.get_mut(&Key::new(3_usize)).map(|v| *v *= 2_f64);
    ///     map.get_mut(&Key::new(4_usize)).map(|v| *v *= 2_f64);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut(&Key::new(1_usize)), Some(&mut 4_f64));
    ///     assert_eq!(map.get_mut(&Key::new(2_usize)), Some(&mut 6_f64));
    ///     assert_eq!(map.get_mut(&Key::new(3_usize)), Some(&mut 8_f64));
    ///     assert_eq!(map.get_mut(&Key::new(4_usize)), None);
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
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
    ///     (Key::new(3_usize), 4_f64),
    /// ]);
    /// let map = het_map.get_map::<f64>().unwrap();
    ///
    /// assert_eq!(map.get_unchecked(&Key::new(1_usize)), &2_f64);
    /// assert_eq!(map.get_unchecked(&Key::new(2_usize)), &3_f64);
    /// assert_eq!(map.get_unchecked(&Key::new(3_usize)), &4_f64);
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
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
    ///     (Key::new(3_usize), 4_f64),
    /// ]);
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(1_usize)), &mut 2_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(2_usize)), &mut 3_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(3_usize)), &mut 4_f64);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     *map.get_mut_unchecked(&Key::new(1_usize)) *= 2_f64;
    ///     *map.get_mut_unchecked(&Key::new(2_usize)) *= 2_f64;
    ///     *map.get_mut_unchecked(&Key::new(3_usize)) *= 2_f64;
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(1_usize)), &mut 4_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(2_usize)), &mut 6_f64);
    ///     assert_eq!(map.get_mut_unchecked(&Key::new(3_usize)), &mut 8_f64);
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
    /// het_map.insert(Key::new(1_usize), String::from("Dark Souls"));
    /// het_map.insert(Key::new(2_usize), String::from("Dark Souls II"));
    /// het_map.insert(Key::new(3_usize), String::from("Dark Souls III"));
    /// het_map.insert(Key::new(4_usize), String::from("Bloodborne"));
    /// het_map.insert(Key::new(5_usize), String::from("Sekiro: Shadows Die Twice"));
    /// het_map.insert(Key::new(6_usize), String::from("Elden Ring"));
    /// het_map.insert(Key::new(7_usize), String::from("Nioh"));
    ///
    /// let expected = [
    ///     &mut String::from("Bloodborne"),
    ///     &mut String::from("Elden Ring"),
    ///     &mut String::from("Nioh"),
    /// ];
    /// let map = het_map.get_map_mut::<String>().unwrap();
    /// let result = map.get_disjoint_mut([
    ///     &Key::new(4_usize),
    ///     &Key::new(0_usize),
    ///     &Key::new(7_usize),
    ///     &Key::new(6_usize),
    /// ]);
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
    ///     assert_eq!(map.get(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.insert(Key::new(1_usize), 2_f64), None);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 1);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.insert(Key::new(2_usize), 3_f64), None);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 2);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.insert(Key::new(3_usize), 4_f64), None);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3_usize)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map::<f64>().unwrap();
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3_usize)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
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
    ///     assert_eq!(map.get(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     map.insert(Key::new(1_usize), 2_f64);
    ///     map.insert(Key::new(2_usize), 3_f64);
    ///     map.insert(Key::new(3_usize), 4_f64);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3_usize)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove(&Key::new(1_usize)), Some(2_f64));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 2);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
    ///     assert_eq!(map.get(&Key::new(3_usize)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove(&Key::new(2_usize)), Some(3_f64));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 1);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get(&Key::new(3_usize)), Some(&4_f64));
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove(&Key::new(3_usize)), Some(4_f64));
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
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
    ///     assert_eq!(map.get(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     map.insert(Key::new(1_usize), 2_f64);
    ///     map.insert(Key::new(2_usize), 3_f64);
    ///     map.insert(Key::new(3_usize), 4_f64);
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 3);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1_usize)), Some((&Key::new(1_usize), &2_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(2_usize)), Some((&Key::new(2_usize), &3_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(3_usize)), Some((&Key::new(3_usize), &4_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove_entry(&Key::new(1_usize)), Some((Key::new(1_usize), 2_f64)));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 2);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(2_usize)), Some((&Key::new(2_usize), &3_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(3_usize)), Some((&Key::new(3_usize), &4_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove_entry(&Key::new(2_usize)), Some((Key::new(2_usize), 3_f64)));
    ///
    ///     assert!(!map.is_empty());
    ///     assert_eq!(map.len(), 1);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(3_usize)), Some((&Key::new(3_usize), &4_f64)));
    ///     assert_eq!(map.get_key_value(&Key::new(4_usize)), None);
    /// }
    /// {
    ///     let map = het_map.get_map_mut::<f64>().unwrap();
    ///     assert_eq!(map.remove_entry(&Key::new(3_usize)), Some((Key::new(3_usize), 4_f64)));
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     assert_eq!(map.get_key_value(&Key::new(1_usize)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(2_usize)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(3_usize)), None);
    ///     assert_eq!(map.get_key_value(&Key::new(4_usize)), None);
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
    ///     (Key::new(1_usize), String::from("foo")),
    ///     (Key::new(2_usize), String::from("bar")),
    ///     (Key::new(3_usize), String::from("baz")),
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
    ///     (Key::new(1_usize), String::from("foo")),
    ///     (Key::new(2_usize), String::from("bar")),
    ///     (Key::new(3_usize), String::from("baz")),
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
    /// let keys = vec![Key::new(1_usize), Key::new(2_usize), Key::new(3_usize)];
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
    /// let keys = vec![Key::new(1_usize), Key::new(2_usize), Key::new(3_usize)];
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
    /// let keys = vec![Key::new(1_usize), Key::new(2_usize), Key::new(3_usize)];
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
    ///     (Key::new(1_usize), String::from("Dark Souls")),
    ///     (Key::new(2_usize), String::from("Dark Souls II")),
    ///     (Key::new(3_usize), String::from("Dark Souls III")),
    ///     (Key::new(4_usize), String::from("Bloodborne")),
    ///     (Key::new(5_usize), String::from("Sekiro: Shadows Die Twice")),
    ///     (Key::new(6_usize), String::from("Elden Ring")),
    ///     (Key::new(7_usize), String::from("Nioh")),
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
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new(1_usize), String::from("Dark Souls")),
    ///     (Key::new(2_usize), String::from("Dark Souls II")),
    ///     (Key::new(3_usize), String::from("Dark Souls III")),
    ///     (Key::new(4_usize), String::from("Bloodborne")),
    ///     (Key::new(5_usize), String::from("Sekiro: Shadows Die Twice")),
    ///     (Key::new(6_usize), String::from("Elden Ring")),
    ///     (Key::new(7_usize), String::from("Nioh")),
    /// ]);
    /// let expected = HomogeneousHashMap::from([
    ///     (Key::new(1_usize), String::from("Dark Souls")),
    ///     (Key::new(2_usize), String::from("Dark Souls II")),
    ///     (Key::new(3_usize), String::from("Dark Souls III")),
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

    /// Returns a moving iterator over the keys of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
    /// #
    /// let mut map: HomogeneousHashMap<String, String> = HomogeneousHashMap::new();
    /// map.insert(
    ///     Key::new(String::from("Geralt of Rivia")),
    ///     String::from("This world doesn't need a hero. It needs a professional."),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Locke Cole")),
    ///     String::from("I’m a treasure hunter, not a thief!"),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Astarion")),
    ///     String::from("You're right. I can be better than him, but I'm not above enjoying this."),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Karlach")),
    ///     String::from("Fine, I guess I’ll just stay here and eat dirt or whatever."),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Beckett")),
    ///     String::from("I consider myself something of an investigator. A scholar of sorts."),
    /// );
    ///
    /// let expected = Vec::from([
    ///     String::from("Astarion"),
    ///     String::from("Beckett"),
    ///     String::from("Geralt of Rivia"),
    ///     String::from("Karlach"),
    ///     String::from("Locke Cole"),
    /// ]);
    /// let result = {
    ///     let mut _result: Vec<String> = map.into_keys().map(|k| k.id().clone()).collect();
    ///     _result.sort();
    ///     _result
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn into_keys(self) -> IntoKeys<K, T> {
        IntoKeys::new(self.inner.into_keys())
    }

    /// Returns a moving iterator over the values of the hash map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
    /// #
    /// let mut map: HomogeneousHashMap<String, String> = HomogeneousHashMap::new();
    /// map.insert(
    ///     Key::new(String::from("Geralt of Rivia")),
    ///     String::from("This world doesn't need a hero. It needs a professional."),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Locke Cole")),
    ///     String::from("I’m a treasure hunter, not a thief!"),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Astarion")),
    ///     String::from("You're right. I can be better than him, but I'm not above enjoying this."),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Karlach")),
    ///     String::from("Fine, I guess I’ll just stay here and eat dirt or whatever."),
    /// );
    /// map.insert(
    ///     Key::new(String::from("Beckett")),
    ///     String::from("I consider myself something of an investigator. A scholar of sorts."),
    /// );
    ///
    /// let expected = Vec::from([
    ///     String::from("Fine, I guess I’ll just stay here and eat dirt or whatever."),
    ///     String::from("I consider myself something of an investigator. A scholar of sorts."),
    ///     String::from("I’m a treasure hunter, not a thief!"),
    ///     String::from("This world doesn't need a hero. It needs a professional."),
    ///     String::from("You're right. I can be better than him, but I'm not above enjoying this."),
    /// ]);
    /// let result = {
    ///     let mut _result: Vec<String> = map.into_values().collect();
    ///     _result.sort();
    ///     _result
    /// };
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn into_values(self) -> IntoValues<K, T> {
        IntoValues::new(self.inner.into_values())
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
    ///     (Key::new(0_usize), String::from("foo")),
    ///     (Key::new(1_usize), String::from("bar")),
    ///     (Key::new(2_usize), String::from("baz")),
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
    ///     (Key::new(0_usize), "foo"),
    ///     (Key::new(1_usize), "bar"),
    ///     (Key::new(2_usize), "baz"),
    ///     (Key::new(3_usize), "quux"),
    ///     (Key::new(4_usize), "quuz"),
    ///     (Key::new(5_usize), "corge"),
    ///     (Key::new(6_usize), "grault"),
    ///     (Key::new(7_usize), "garply"),
    ///     (Key::new(8_usize), "waldo"),
    /// ]);
    /// let mut expected_het_map = HeterogeneousHashMap::new();
    /// expected_het_map.extend(Vec::from([
    ///     (Key::new(0_usize), "foo"),
    ///     (Key::new(1_usize), "bar"),
    ///     (Key::new(2_usize), "baz"),
    ///     (Key::new(5_usize), "corge"),
    ///     (Key::new(8_usize), "waldo"),
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
    ///     (Key::new(0_usize), 1_i32),
    ///     (Key::new(1_usize), 2_i32),
    ///     (Key::new(2_usize), 3_i32),
    ///     (Key::new(3_usize), 4_i32),
    ///     (Key::new(4_usize), 5_i32),
    ///     (Key::new(5_usize), 6_i32),
    /// ]);
    /// map.reserve(10);
    ///
    /// assert!(map.capacity() >= map.len() + 10);
    ///
    /// let old_capacity = map.capacity();
    /// map.extend([
    ///     (Key::new(6_usize), 7_i32),
    ///     (Key::new(7_usize), 8_i32),
    ///     (Key::new(8_usize), 9_i32),
    ///     (Key::new(9_usize), 10_i32),
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
    ///     (Key::new(0_usize), 1_i32),
    ///     (Key::new(1_usize), 2_i32),
    ///     (Key::new(2_usize), 3_i32),
    ///     (Key::new(3_usize), 4_i32),
    ///     (Key::new(4_usize), 5_i32),
    ///     (Key::new(5_usize), 6_i32),
    /// ]);
    /// let result = map.try_reserve(10);
    ///
    /// assert!(result.is_ok());
    /// assert!(map.capacity() >= map.len() + 10);
    ///
    /// let old_capacity = map.capacity();
    /// map.extend([
    ///     (Key::new(6_usize), 7_i32),
    ///     (Key::new(7_usize), 8_i32),
    ///     (Key::new(8_usize), 9_i32),
    ///     (Key::new(9_usize), 10_i32),
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
    /// map.extend([(Key::new(0_usize), 1_i32), (Key::new(1_usize), 2_i32), (Key::new(2_usize), 3_i32)]);
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
    /// map.extend([(Key::new(0_usize), 1_i32), (Key::new(1_usize), 2_i32), (Key::new(2_usize), 3_i32)]);
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

impl<K, T, S> IntoIterator for HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    type Item = (Key<K, T>, T);
    type IntoIter = IntoIter<Key<K, T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.inner.into_iter())
    }
}

impl<'a, K, T, S> IntoIterator for &'a HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    type Item = (&'a Key<K, T>, &'a T);
    type IntoIter = Iter<'a, K, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, T, S> IntoIterator for &'a mut HomogeneousHashMap<K, T, S>
where
    K: any::Any,
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    type Item = (&'a Key<K, T>, &'a mut T);
    type IntoIter = IterMut<'a, K, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
