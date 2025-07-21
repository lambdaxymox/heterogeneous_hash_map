use crate::homogeneous_hash_map::HomogeneousHashMap;
use crate::iterator::TypeMetadataIter;
use crate::key::Key;
use crate::metadata::TypeMetadata;

use core::any;
use core::fmt;
use core::marker;
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
///     (Key::new(1_usize), 2_i32),
///     (Key::new(2_usize), 3_i32),
///     (Key::new(3_usize), 5_i32),
/// ]);
/// het_map.extend((0..=15).map(|i| (Key::new(i), 1_u16 << i)));
///
/// // A heterogeneous hash map inserts a new type automatically when inserting elements of a new type.
/// het_map.extend::<_, f64>([
///     (Key::new(2_usize),  champernowne_constant),
///     (Key::new(3_usize),  liouville_number),
///     (Key::new(5_usize),  feigenbaum_delta),
///     (Key::new(7_usize),  khinchin_constant),
///     (Key::new(11_usize), gelfond_schneider_constant),
/// ]);
///
/// // Checking the contents of the heterogeneous hash map before manipulation.
/// {
///     assert_eq!(het_map.len_types(), 3);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), Some(5));
///
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&5_i32));
///
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2_usize)),  Some(&champernowne_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3_usize)),  Some(&liouville_number));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5_usize)),  Some(&feigenbaum_delta));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7_usize)),  Some(&khinchin_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11_usize)), Some(&gelfond_schneider_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13_usize)), None);
/// }
///
/// // Removing an element of a specific type from a heterogeneous hash map.
/// {
///     assert_eq!(het_map.remove::<f64, _>(&Key::new(5_usize)), Some(feigenbaum_delta));
///
///     assert_eq!(het_map.len_types(), 3);
///     assert_eq!(het_map.len::<i32>(), Some(3));
///     assert_eq!(het_map.len::<u16>(), Some(16));
///     assert_eq!(het_map.len::<f64>(), Some(4));
///
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&5_i32));
///
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2_usize)),  Some(&champernowne_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3_usize)),  Some(&liouville_number));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7_usize)),  Some(&khinchin_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11_usize)), Some(&gelfond_schneider_constant));
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13_usize)), None);
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
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&5_i32));
///
///     // The `u16` entries remain untouched.
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     // Every `f64` entry was removed from the map.
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11_usize)), None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13_usize)), None);
/// }
///
/// // Inserting one value of a new data type into a heterogeneous hash map.
/// {
///     assert!(!het_map.contains_type::<f64>());
///
///     assert_eq!(het_map.insert(Key::new(13_usize), f64::consts::PI), None);
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
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&2_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&3_i32));
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), Some(&5_i32));
///
///     // The `u16` entries remain untouched.
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), Some(&(1_u16 << i)));
///     }
///
///     // The previous `f64` entries prior to the call to `remove_type` no longer exist, but the
///     // newly inserted one does.
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11_usize)), None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13_usize)), Some(&f64::consts::PI));
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
///     assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
///     assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
///     assert_eq!(het_map.get::<i32, _>(&Key::new(3_usize)), None);
///
///     // Every value of every type `u16` is gone.
///     for i in 0..=15 {
///         assert_eq!(het_map.get::<u16, _>(&Key::new(i)), None);
///     }
///
///     // Every value of type `f64` is gone.
///     assert_eq!(het_map.get::<f64, _>(&Key::new(2_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(3_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(5_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(7_usize)),  None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(11_usize)), None);
///     assert_eq!(het_map.get::<f64, _>(&Key::new(13_usize)), None);
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
    /// het_map.insert::<i32>(Key::new(0_usize), 1_i32);
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
    /// het_map.insert::<i32>(Key::new(0_usize), 1_i32);
    /// het_map.insert::<i32>(Key::new(1_usize), 2_i32);
    /// het_map.insert::<i32>(Key::new(1_usize), 3_i32);
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
    ///     (Key::new(0_usize), 2_i32),
    ///     (Key::new(1_usize), 3_i32),
    ///     (Key::new(2_usize), 5_i32),
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
    ///     (Key::new(0_usize), 2_i32),
    ///     (Key::new(1_usize), 3_i32),
    ///     (Key::new(2_usize), 5_i32),
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
    ///     (Key::new(0_usize), 2_i32),
    ///     (Key::new(1_usize), 3_i32),
    ///     (Key::new(2_usize), 5_i32),
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
    ///     (Key::new(0_usize), 2_i32),
    ///     (Key::new(1_usize), 3_i32),
    ///     (Key::new(2_usize), 5_i32),
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
    ///     (Key::new(0_usize), 1_f64),
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
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
    ///     (Key::new(0_usize), 1_f64),
    ///     (Key::new(1_usize), 2_f64),
    ///     (Key::new(2_usize), 3_f64),
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
    ///     assert_eq!(map.get(&Key::new(0_usize)), Some(&1_f64));
    ///     assert_eq!(map.get(&Key::new(1_usize)), Some(&2_f64));
    ///     assert_eq!(map.get(&Key::new(2_usize)), Some(&3_f64));
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
    /// het_map.extend([(Key::new(1_usize), 2_f64), (Key::new(2_usize), 3_f64), (Key::new(3_usize), 4_f64)]);
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
    ///     (Key::new(1_usize), 3_i32),
    ///     (Key::new(2_usize), 5_i32),
    ///     (Key::new(3_usize), 7_i32),
    /// ];
    /// let values2 = [
    ///     (Key::new(1_usize), 2_u64),
    /// ];
    /// let values3 = [
    ///     (Key::new(1_usize), String::from("foo")),
    ///     (Key::new(2_usize), String::from("bar")),
    ///     (Key::new(3_usize), String::from("baz")),
    ///     (Key::new(4_usize), String::from("quux")),
    ///     (Key::new(5_usize), String::from("garply")),
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
    /// het_map.insert::<i32>(Key::new(0_usize), 1_i32);
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
    ///     (Key::new(0_usize), f64::consts::SQRT_2),
    ///     (Key::new(1_usize), f64::consts::PI),
    ///     (Key::new(2_usize), f64::consts::E),
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
    ///     (Key::new(0_usize), f64::consts::SQRT_2),
    ///     (Key::new(1_usize), f64::consts::PI),
    ///     (Key::new(2_usize), f64::consts::E),
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
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(0_usize)));
    /// ```
    ///
    /// Querying a nonempty heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Key};
    /// #
    /// let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(0_usize)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(0_usize)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    ///
    /// het_map.insert(Key::new(0_usize), i32::MAX);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(0_usize)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
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
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    ///
    /// het_map.insert(Key::new(0_usize), i32::MAX);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), Some(&i32::MAX));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
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
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
    ///
    /// het_map.insert(Key::new(0_usize), i32::MAX);
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(0_usize)), Some((&Key::new(0_usize), &i32::MAX)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&Key::new(1_usize)), None);
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
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    ///
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    ///
    /// het_map.insert(Key::new(0_usize), i32::MAX);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), Some(&i32::MAX));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
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
    /// let result = het_map.get_disjoint_mut::<String, _, 4>([
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
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    ///
    /// het_map.insert(Key::new(1_usize), 5_i32);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
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
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    ///
    /// het_map.insert(Key::new(1_usize), 3_i32);
    /// het_map.insert(Key::new(2_usize), 5_i32);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    ///
    /// assert_eq!(het_map.remove::<i32, _>(&Key::new(1_usize)), Some(3_i32));
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
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
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    ///
    /// het_map.insert(Key::new(1_usize), 3_i32);
    /// het_map.insert(Key::new(2_usize), 5_i32);
    ///
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
    ///
    /// assert_eq!(het_map.remove_entry::<i32, _>(&Key::new(1_usize)), Some((Key::new(1_usize), 3_i32)));
    ///
    /// assert!(!het_map.contains_key::<i32, _>(&Key::new(1_usize)));
    /// assert!(het_map.contains_key::<i32, _>(&Key::new(2_usize)));
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
    /// het_map.insert(Key::new(1_usize), i32::MAX);
    ///
    /// assert_eq!(het_map.get_unchecked::<i32, _>(&Key::new(1_usize)), &i32::MAX);
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
    /// het_map.insert(Key::new(1_usize), i32::MAX);
    ///
    /// assert_eq!(het_map.get_mut_unchecked::<i32, _>(&Key::new(1_usize)), &i32::MAX);
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
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), None);
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), None);
    ///
    /// assert_eq!(het_map.get::<String, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(5_usize)), None);
    ///
    /// het_map.extend([
    ///     (Key::new(0_usize), 1_i32),
    ///     (Key::new(1_usize), 2_i32),
    ///     (Key::new(2_usize), 3_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), Some(&1_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&2_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&3_i32));
    ///
    /// assert_eq!(het_map.get::<String, _>(&Key::new(0_usize)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), None);
    /// assert_eq!(het_map.get::<String, _>(&Key::new(5_usize)), None);
    ///
    /// het_map.extend([
    ///     (Key::new(0_usize), String::from("foo")),
    ///     (Key::new(3_usize), String::from("bar")),
    ///     (Key::new(5_usize), String::from("baz")),
    /// ]);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(0_usize)), Some(&1_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(1_usize)), Some(&2_i32));
    /// assert_eq!(het_map.get::<i32, _>(&Key::new(2_usize)), Some(&3_i32));
    ///
    /// assert_eq!(het_map.get::<String, _>(&Key::new(0_usize)), Some(&String::from("foo")));
    /// assert_eq!(het_map.get::<String, _>(&Key::new(3_usize)), Some(&String::from("bar")));
    /// assert_eq!(het_map.get::<String, _>(&Key::new(5_usize)), Some(&String::from("baz")));
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
            assert_eq!(metadata.type_id(), *type_id);
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
