use crate::key::Key;

use core::any;
use core::fmt;

/// A view into a single entry in a [`HomogeneousHashMap`] or a [`HeterogeneousHashMap`], which may
/// be occupied or vacant.
///
/// Entries are obtained by using the [`HomogeneousMap::entry`], [`HeterogeneousHashMap::entry`],
/// and [`HeterogeneousHashMap::entry_or_insert_type`] methods.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
/// # use core::any::Any;
/// #
/// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
///     match entry {
///         Entry::Occupied(occupied_entry) => occupied_entry,
///         _ => panic!("This method only destructures occupied entries")
///     }
/// }
///
/// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
///     (Key::new("foo"),  1_i32),
///     (Key::new("bar"),  2_i32),
///     (Key::new("baz"),  4_i32),
///     (Key::new("quux"), 8_i32),
/// ]);
///
/// assert_eq!(map.len(), 4);
///
/// assert_eq!(map.entry(Key::new("foo")).key(), &Key::new("foo"));
/// {
///     let entry = map.entry(Key::new("foo"));
///     let occupied_entry = into_occupied(entry);
///     assert_eq!(occupied_entry.get(), &1_i32);
/// }
/// assert_eq!(map.entry(Key::new("bar")).key(), &Key::new("bar"));
/// {
///     let entry = map.entry(Key::new("bar"));
///     let occupied_entry = into_occupied(entry);
///     assert_eq!(occupied_entry.get(), &2_i32);
/// }
/// assert_eq!(map.entry(Key::new("baz")).key(), &Key::new("baz"));
/// {
///     let entry = map.entry(Key::new("baz"));
///     let occupied_entry = into_occupied(entry);
///     assert_eq!(occupied_entry.get(), &4_i32);
/// }
/// assert_eq!(map.entry(Key::new("quux")).key(), &Key::new("quux"));
/// {
///     let entry = map.entry(Key::new("quux"));
///     let occupied_entry = into_occupied(entry);
///     assert_eq!(occupied_entry.get(), &8_i32);
/// }
///
/// // Vacant entries also return their keys.
/// assert_eq!(map.entry(Key::new("quuz")).key(), &Key::new("quuz"));
/// assert_eq!(map.entry(Key::new("garply")).key(), &Key::new("garply"));
///
/// assert_eq!(map.len(), 4);
///
/// map.entry(Key::new("quuz")).insert_entry(16_i32);
///
/// assert_eq!(map.len(), 5);
///
/// assert_eq!(map.entry(Key::new("quuz")).key(), &Key::new("quuz"));
/// {
///     let entry = map.entry(Key::new("quuz"));
///     let occupied_entry = into_occupied(entry);
///     assert_eq!(occupied_entry.get(), &16_i32);
/// }
/// assert_eq!(map.entry(Key::new("garply")).key(), &Key::new("garply"));
/// ```
pub enum Entry<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, T>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, T>),
}

impl<'a, K, T> Entry<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Sets the value of the entry (after inserting the entry if it is vacant), and returning an
    /// [`opaque::index_map::map::OccupiedEntry`].
    ///
    /// This method behaves as follows:
    ///
    /// * If the entry is occupied, this method replaces the old value with the new value in the
    ///   entry, and returns an occupied entry.
    /// * If the entry is vacant, the entry is inserted into the map, so the resulting and the
    ///   method returns an occupied entry containing the value `value` and the key from the
    ///   original vacant entry.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut map = HomogeneousHashMap::new();
    ///
    /// assert_eq!(map.len(), 0);
    /// {
    ///     let new_entry = map.entry(Key::new("foo"));
    ///     let occupied_entry = new_entry.insert_entry(1_i32);
    ///     assert_eq!(occupied_entry.key(), &Key::new("foo"));
    ///     assert_eq!(occupied_entry.get(), &1_i32);
    /// }
    /// assert_eq!(map.len(), 1);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    ///
    /// assert_eq!(het_map.len::<i32>(), Some(0));
    /// {
    ///     let new_entry = het_map.entry::<i32>(Key::new("foo")).unwrap();
    ///     let occupied_entry = new_entry.insert_entry(1_i32);
    ///     assert_eq!(occupied_entry.key(), &Key::new("foo"));
    ///     assert_eq!(occupied_entry.get(), &1_i32);
    /// }
    /// assert_eq!(het_map.len::<i32>(), Some(1));
    /// ```
    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, K, T> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }

    /// Ensures a value is in the entry by inserting the default value if it is empty, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// If the entry is occupied, this method does nothing and returns a mutable reference to its
    /// value. If the entry is vacant, this method inserts the provided default value and returns a
    /// mutable reference to the entry's value.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut map = HomogeneousHashMap::new();
    /// map.entry(Key::new("foo")).or_insert(3_i32);
    ///
    /// assert_eq!(map.get(&"foo"), Some(&3_i32));
    ///
    /// // This does nothing since the entry with key `"foo"` already exists in the map.
    /// map.entry(Key::new("foo")).or_insert(10_i32);
    ///
    /// assert_ne!(map.get(&"foo"), Some(&10_32));
    /// assert_eq!(map.get(&"foo"), Some(&3_i32));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.insert_type::<i32>();
    /// {
    ///     let entry = het_map.entry::<i32>(Key::new("foo")).unwrap();
    ///     entry.or_insert(3_i32);
    /// }
    /// assert_eq!(het_map.get::<i32, _>(&"foo"), Some(&3_i32));
    ///
    /// // This does nothing since the entry with key `"foo"` already exists in the map.
    /// {
    ///     let entry = het_map.entry::<i32>(Key::new("foo")).unwrap();
    ///     entry.or_insert(10_i32);
    /// }
    /// assert_ne!(het_map.get::<i32, _>(&"foo"), Some(&10_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"foo"), Some(&3_i32));
    /// ```
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if it is
    /// empty, and returns a mutable reference to the value in the entry.
    ///
    /// This method behaves as follows:
    ///
    /// * If the entry is occupied, this method does nothing and returns a mutable reference to its
    ///   value.
    /// * If the entry is vacant, this method inserts the result of calling the provided function
    ///   `call` and returns a mutable reference to the entry's value.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut map = HomogeneousHashMap::new();
    /// let default_value = "garply";
    /// let another_value = "corge";
    /// map.entry(Key::new("foo")).or_insert_with(|| default_value);
    ///
    /// assert_eq!(map.get(&"foo"), Some(&default_value));
    ///
    /// // This does nothing since the entry with key `"foo"` already exists in the map.
    /// map.entry(Key::new("foo")).or_insert_with(|| another_value);
    ///
    /// assert_ne!(map.get(&"foo"), Some(&another_value));
    /// assert_eq!(map.get(&"foo"), Some(&default_value));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry, HeterogeneousHashMap};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let default_value = "garply";
    /// let another_value = "corge";
    /// het_map.insert_type::<&str>();
    /// het_map.entry_or_insert_type(Key::new("foo")).or_insert_with(|| default_value);
    ///
    /// assert_eq!(het_map.get::<&str, _>(&"foo"), Some(&default_value));
    ///
    /// // This does nothing since the entry with key `"foo"` already exists in the map.
    /// het_map.entry_or_insert_type(Key::new("foo")).or_insert_with(|| another_value);
    ///
    /// assert_ne!(het_map.get::<&str, _>(&"foo"), Some(&another_value));
    /// assert_eq!(het_map.get::<&str, _>(&"foo"), Some(&default_value));
    /// ```
    pub fn or_insert_with<F>(self, call: F) -> &'a mut T
    where
        F: FnOnce() -> T,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(call()),
        }
    }

    /// Ensures a value is in the entry, using the provided default function if necessary.
    ///
    /// This method behaves as follows:
    ///
    /// * If the entry is occupied, this method does nothing, and returns a mutable reference to
    ///   its value.
    /// * Is the entry is vacant, this method inserts the result of the default function.
    ///
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the [`entry`] method call.
    ///
    /// [`entry`]: TypeProjectedIndexMap::entry
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut map = HomogeneousHashMap::new();
    /// let default_value = "garply";
    /// let another_value = "corge";
    /// let func = |key: &Key<&str, &str>| if key == &Key::new("foo") { default_value } else { another_value };
    ///
    /// map.entry(Key::new("foo")).or_insert_with_key(func);
    ///
    /// assert_eq!(map.get(&"foo"), Some(&default_value));
    /// assert_ne!(map.get(&"foo"), Some(&another_value));
    ///
    /// map.entry(Key::new("bar")).or_insert_with_key(func);
    ///
    /// assert_eq!(map.get(&"bar"), Some(&another_value));
    /// assert_ne!(map.get(&"bar"), Some(&default_value));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let default_value = "garply";
    /// let another_value = "corge";
    /// het_map.insert_type::<&str>();
    /// let func = |key: &Key<&str, &str>| if key == &Key::new("foo") { default_value } else { another_value };
    ///
    /// het_map.entry_or_insert_type(Key::new("foo")).or_insert_with_key(func);
    ///
    /// assert_eq!(het_map.get::<&str, _>(&"foo"), Some(&default_value));
    /// assert_ne!(het_map.get::<&str, _>(&"foo"), Some(&another_value));
    ///
    /// het_map.entry_or_insert_type(Key::new("bar")).or_insert_with_key(func);
    ///
    /// assert_eq!(het_map.get::<&str, _>(&"bar"), Some(&another_value));
    /// assert_ne!(het_map.get::<&str, _>(&"bar"), Some(&default_value));
    /// ```
    pub fn or_insert_with_key<F>(self, call: F) -> &'a mut T
    where
        F: FnOnce(&Key<K, T>) -> T,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = call(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Gets a reference to the entry's key in the hash map.
    ///
    /// This method behaves as follows:
    ///
    /// * If the entry is occupied, this method returns the key stored in the hash map for that
    ///   entry.
    /// * If the entry is vacant, this method returns the key that was used to search for the entry
    ///   in the hash map.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"), 1_i32),
    ///     (Key::new("bar"), 2_i32),
    ///     (Key::new("baz"), 3_i32),
    /// ]);
    ///
    /// assert_eq!(map.entry(Key::new("foo")).key(), &"foo");
    /// assert_eq!(map.entry(Key::new("bar")).key(), &"bar");
    /// assert_eq!(map.entry(Key::new("baz")).key(), &"baz");
    ///
    /// // Vacant entries have keys too.
    /// assert_eq!(map.entry(Key::new("quux")).key(), &"quux");
    /// ```
    ///
    /// Using this map with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"), 1_i32),
    ///     (Key::new("bar"), 2_i32),
    ///     (Key::new("baz"), 3_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.entry_or_insert_type::<i32>(Key::new("foo")).key(), &"foo");
    /// assert_eq!(het_map.entry_or_insert_type::<i32>(Key::new("bar")).key(), &"bar");
    /// assert_eq!(het_map.entry_or_insert_type::<i32>(Key::new("baz")).key(), &"baz");
    ///
    /// // Vacant entries have keys too.
    /// assert_eq!(het_map.entry_or_insert_type::<i32>(Key::new("quux")).key(), &"quux");
    /// ```
    pub fn key(&self) -> &Key<K, T> {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in place mutable access to an occupied entry before any potential insertions into
    /// the hash map.
    ///
    /// If the entry `self` is vacant, this method does nothing.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut map = HomogeneousHashMap::new();
    /// map.entry(Key::new("foo")).and_modify(|e| { *e += 1 }).or_insert(42_i32);
    ///
    /// assert_eq!(map.get(&"foo"), Some(&42_i32));
    ///
    /// map.entry(Key::new("foo")).and_modify(|e| { *e += 1 }).or_insert(42_i32);
    ///
    /// assert_eq!(map.get(&"foo"), Some(&43_i32));
    ///
    /// // The `and_modify` method has no effect on vacant entries.
    /// map.entry(Key::new(&"bar")).and_modify(|e| *e = 55_i32).or_insert(44_i32);
    ///
    /// assert_eq!(map.get(&"bar"), Some(&44_i32));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.entry_or_insert_type::<i32>(Key::new("foo")).and_modify(|e| { *e += 1 }).or_insert(42_i32);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&"foo"), Some(&42_i32));
    ///
    /// het_map.entry_or_insert_type::<i32>(Key::new("foo")).and_modify(|e| { *e += 1 }).or_insert(42_i32);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&"foo"), Some(&43_i32));
    ///
    /// // The `and_modify` method has no effect on vacant entries.
    /// het_map.entry_or_insert_type::<i32>(Key::new(&"bar")).and_modify(|e| *e = 55_i32).or_insert(44_i32);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&"bar"), Some(&44_i32));
    /// ```
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        if let Entry::Occupied(entry) = &mut self {
            f(entry.get_mut());
        }

        self
    }

    /// Ensures that a value is in the entry by inserting the default value if necessary.
    ///
    /// This method behaves as follows:
    ///
    /// * If the entry is vacant, this method inserts the default value and returns a mutable
    ///   reference to the value in the entry.
    /// * If the entry is occupied, the method does nothing and returns a mutable reference to the
    ///   value in the entry.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// # #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// enum SpatialPartition {
    ///     QuadTree,
    ///     OctTree,
    ///     BVH,
    ///     Grid,
    /// }
    /// # impl Default for SpatialPartition {
    /// #     fn default() -> Self {
    /// #         SpatialPartition::QuadTree
    /// #     }
    /// # }
    /// #
    ///
    /// let mut map: HomogeneousHashMap<&str, SpatialPartition> = HomogeneousHashMap::new();
    ///
    /// assert!(!map.contains_key(&"foo"));
    ///
    /// map.entry(Key::new("foo")).or_default();
    ///
    /// assert!(map.contains_key(&"foo"));
    ///
    /// assert_eq!(map.get(&"foo"), Some(&SpatialPartition::default()));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// #
    /// # #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// enum SpatialPartition {
    ///     QuadTree,
    ///     OctTree,
    ///     BVH,
    ///     Grid,
    /// }
    /// # impl Default for SpatialPartition {
    /// #     fn default() -> Self {
    /// #         SpatialPartition::QuadTree
    /// #     }
    /// # }
    /// #
    ///
    /// let mut het_map: HeterogeneousHashMap<&str> = HeterogeneousHashMap::new();
    ///
    /// assert!(!het_map.contains_key::<SpatialPartition, _>(&"foo"));
    ///
    /// het_map.entry_or_insert_type::<SpatialPartition>(Key::new("foo")).or_default();
    ///
    /// assert!(het_map.contains_key::<SpatialPartition, _>(&"foo"));
    ///
    /// assert_eq!(het_map.get::<SpatialPartition, _>(&"foo"), Some(&SpatialPartition::default()));
    /// ```
    pub fn or_default(self) -> &'a mut T
    where
        T: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(T::default()),
        }
    }
}

impl<K, T> fmt::Debug for Entry<'_, K, T>
where
    K: any::Any + fmt::Debug,
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = formatter.debug_tuple("Entry");
        match self {
            Entry::Vacant(v) => tuple.field(v),
            Entry::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

/// A view into an occupied entry in a [`HomogeneousHashMap`] or [`HeterogeneousHashMap`]. It is
/// part of the [`Entry`] sum type.
pub struct OccupiedEntry<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    inner: opaque::index_map::map::OccupiedEntry<'a, Key<K, T>, T>,
}

impl<'a, K, T> OccupiedEntry<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs a new occupied entry.
    #[inline]
    pub(crate) const fn new(inner: opaque::index_map::map::OccupiedEntry<'a, Key<K, T>, T>) -> Self {
        Self { inner }
    }

    /// Gets a reference to the key stored in the occupied entry in the hash map.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(into_occupied(map.entry(Key::new("foo"))).key(),  &"foo");
    /// assert_eq!(into_occupied(map.entry(Key::new("bar"))).key(),  &"bar");
    /// assert_eq!(into_occupied(map.entry(Key::new("baz"))).key(),  &"baz");
    /// assert_eq!(into_occupied(map.entry(Key::new("quux"))).key(), &"quux");
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("foo"))).key(),  &"foo");
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("bar"))).key(),  &"bar");
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("baz"))).key(),  &"baz");
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("quux"))).key(), &"quux");
    /// ```
    pub fn key(&self) -> &Key<K, T> {
        self.inner.key()
    }

    /// Gets a reference to the occupied entry's value in the hash map.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(into_occupied(map.entry(Key::new("foo"))).get(),  &1_i32);
    /// assert_eq!(into_occupied(map.entry(Key::new("bar"))).get(),  &2_i32);
    /// assert_eq!(into_occupied(map.entry(Key::new("baz"))).get(),  &3_i32);
    /// assert_eq!(into_occupied(map.entry(Key::new("quux"))).get(), &4_i32);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("foo"))).get(),  &1_i32);
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("bar"))).get(),  &2_i32);
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("baz"))).get(),  &3_i32);
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("quux"))).get(), &4_i32);
    /// ```
    pub fn get(&self) -> &T {
        self.inner.get()
    }

    /// Gets a mutable reference to the occupied entry's value in the hash map.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(into_occupied(map.entry(Key::new("foo"))).get_mut(),  &mut 1_i32);
    /// assert_eq!(into_occupied(map.entry(Key::new("bar"))).get_mut(),  &mut 2_i32);
    /// assert_eq!(into_occupied(map.entry(Key::new("baz"))).get_mut(),  &mut 3_i32);
    /// assert_eq!(into_occupied(map.entry(Key::new("quux"))).get_mut(), &mut 4_i32);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("foo"))).get_mut(),  &mut 1_i32);
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("bar"))).get_mut(),  &mut 2_i32);
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("baz"))).get_mut(),  &mut 3_i32);
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("quux"))).get_mut(), &mut 4_i32);
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Converts the occupied entry into a mutable reference to the value in the entry with a
    /// lifetime bound to the hash map itself.
    ///
    /// Use [`get_mut`] to get multiple references to the occupied entry.
    ///
    /// [`get_mut`]: OccupiedEntry::get_mut
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// {
    ///     let mut occupied_entry = into_occupied(map.entry(Key::new("bar")));
    ///     let result = occupied_entry.into_mut();
    ///
    ///     assert_eq!(result, &2_i32);
    ///
    ///     *result = i32::MAX;
    /// }
    /// assert_eq!(into_occupied(map.entry(Key::new("bar"))).get(), &i32::MAX);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// {
    ///     let mut occupied_entry = into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("bar")));
    ///     let result = occupied_entry.into_mut();
    ///
    ///     assert_eq!(result, &2_i32);
    ///
    ///     *result = i32::MAX;
    /// }
    /// assert_eq!(into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("bar"))).get(), &i32::MAX);
    /// ```
    pub fn into_mut(self) -> &'a mut T {
        self.inner.into_mut()
    }

    /// Sets the value of the occupied entry to a new value, and returns the old value.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    ///
    /// let mut occupied_entry = into_occupied(map.entry(Key::new("bar")));
    /// let result = occupied_entry.insert(i32::MAX);
    ///
    /// assert_eq!(result, 2_i32);
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &i32::MAX)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(het_map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    ///
    /// let mut occupied_entry = into_occupied(het_map.entry_or_insert_type(Key::new("bar")));
    /// let result = occupied_entry.insert(i32::MAX);
    ///
    /// assert_eq!(result, 2_i32);
    ///
    /// assert_eq!(het_map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &i32::MAX)));
    /// assert_eq!(het_map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// ```
    pub fn insert(&mut self, value: T) -> T {
        self.inner.insert(value)
    }

    /// Removes the occupied entry from the hash map, and returns the value of the entry.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    ///
    /// let occupied_entry = into_occupied(map.entry(Key::new("bar")));
    /// let result = occupied_entry.remove();
    ///
    /// assert_eq!(result, 2_i32);
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"quux"), Some((&Key::new("quux"), &4_i32)));
    ///
    /// let occupied_entry = into_occupied(het_map.entry_or_insert_type::<i32>(Key::new("bar")));
    /// let result = occupied_entry.remove();
    ///
    /// assert_eq!(result, 2_i32);
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// ```
    pub fn remove(self) -> T {
        self.inner.swap_remove_entry().1
    }

    /// Removes the occupied entry from the hash map, and returns the key-value pair for the entry.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    ///
    /// let occupied_entry = into_occupied(map.entry(Key::new("bar")));
    /// let result = occupied_entry.remove_entry();
    ///
    /// assert_eq!(result, (Key::new("bar"), 2_i32));
    ///
    /// assert_eq!(map.get_key_value(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(map.get_key_value(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(map.get_key_value(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, OccupiedEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_occupied<K: Any, T: Any>(entry: Entry<'_, K, T>) -> OccupiedEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Occupied(occupied_entry) => occupied_entry,
    ///         _ => panic!("This method only destructures occupied entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"bar"),  Some((&Key::new("bar"),  &2_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"quux"), Some((&Key::new("quux"), &4_i32)));
    ///
    /// let occupied_entry = into_occupied(het_map.entry_or_insert_type(Key::new("bar")));
    /// let result = occupied_entry.remove_entry();
    ///
    /// assert_eq!(result, (Key::new("bar"), 2_i32));
    ///
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"foo"),  Some((&Key::new("foo"),  &1_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"baz"),  Some((&Key::new("baz"),  &3_i32)));
    /// assert_eq!(het_map.get_key_value::<i32, _>(&"quux"), Some((&Key::new("quux"), &4_i32)));
    /// ```
    pub fn remove_entry(self) -> (Key<K, T>, T) {
        self.inner.swap_remove_entry()
    }
}

impl<K, T> fmt::Debug for OccupiedEntry<'_, K, T>
where
    K: any::Any + fmt::Debug,
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

/// A view into a vacant entry in a [`HomogeneousHashMap`] or [`HeterogeneousHashMap`]. It is
/// part of the [`Entry`] sum type.
pub struct VacantEntry<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    inner: opaque::index_map::map::VacantEntry<'a, Key<K, T>, T>,
}

impl<'a, K, T> VacantEntry<'a, K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs a new vacant entry.
    #[inline]
    pub(crate) const fn new(inner: opaque::index_map::map::VacantEntry<'a, Key<K, T>, T>) -> Self {
        Self { inner }
    }

    /// Gets a reference to the key that would be used when inserting a value through the vacant
    /// entry.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::new();
    /// let vacant_entry = into_vacant(map.entry(Key::new("foo")));
    /// let expected = Key::new("foo");
    /// let result = vacant_entry.key();
    ///
    /// assert_eq!(result, &expected);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut map = HeterogeneousHashMap::new();
    /// let vacant_entry = into_vacant(map.entry_or_insert_type::<i32>(Key::new("foo")));
    /// let expected = Key::new("foo");
    /// let result = vacant_entry.key();
    ///
    /// assert_eq!(result, &expected);
    /// ```
    pub fn key(&self) -> &Key<K, T> {
        self.inner.key()
    }

    /// Takes ownership of the key for this vacant entry.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::new();
    /// let vacant_entry = into_vacant(map.entry(Key::new("foo")));
    /// let expected = Key::new("foo");
    /// let result = vacant_entry.into_key();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// let vacant_entry = into_vacant(het_map.entry_or_insert_type::<i32>(Key::new("foo")));
    /// let expected = Key::new("foo");
    /// let result = vacant_entry.into_key();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn into_key(self) -> Key<K, T> {
        self.inner.into_key()
    }

    /// Sets the value of the vacant entry, then returns a mutable reference to the value.
    ///
    /// # Examples
    ///
    /// Using this method on a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(map.get(&"foo"),  Some(&1_i32));
    /// assert_eq!(map.get(&"bar"),  Some(&2_i32));
    /// assert_eq!(map.get(&"baz"),  Some(&3_i32));
    /// assert_eq!(map.get(&"quux"), Some(&4_i32));
    ///
    /// let result = into_vacant(map.entry(Key::new("corge"))).insert(i32::MAX);
    ///
    /// assert_eq!(result, &i32::MAX);
    /// ```
    ///
    /// Using this method on a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&"foo"),  Some(&1_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"bar"),  Some(&2_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"baz"),  Some(&3_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"quux"), Some(&4_i32));
    ///
    /// let result = into_vacant(het_map.entry_or_insert_type::<i32>(Key::new("corge"))).insert(i32::MAX);
    ///
    /// assert_eq!(result, &i32::MAX);
    /// ```
    pub fn insert(self, value: T) -> &'a mut T {
        self.inner.insert(value)
    }

    /// Sets the value of the vacant entry in the hash map, then returns an occupied entry
    /// corresponding to the key-value pair now stored in the hash map.
    ///
    /// # Examples
    ///
    /// Using this method with a homogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HomogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(map.get(&"foo"),  Some(&1_i32));
    /// assert_eq!(map.get(&"bar"),  Some(&2_i32));
    /// assert_eq!(map.get(&"baz"),  Some(&3_i32));
    /// assert_eq!(map.get(&"quux"), Some(&4_i32));
    ///
    /// let occupied_entry = into_vacant(map.entry(Key::new("corge"))).insert_entry(i32::MAX);
    ///
    /// assert_eq!(occupied_entry.key(), &Key::new("corge"));
    /// assert_eq!(occupied_entry.get(), &i32::MAX);
    /// ```
    ///
    /// Using this method with a heterogeneous hash map.
    ///
    /// ```
    /// # use heterogeneous_hash_map::{HeterogeneousHashMap, Entry, Key, VacantEntry};
    /// # use core::any::Any;
    /// #
    /// fn into_vacant<K: Any, T: Any>(entry: Entry<'_, K, T>) -> VacantEntry<'_, K, T> {
    ///     match entry {
    ///         Entry::Vacant(vacant_entry) => vacant_entry,
    ///         _ => panic!("This method only destructures vacant entries")
    ///     }
    /// }
    ///
    /// let mut het_map = HeterogeneousHashMap::new();
    /// het_map.extend([
    ///     (Key::new("foo"),  1_i32),
    ///     (Key::new("bar"),  2_i32),
    ///     (Key::new("baz"),  3_i32),
    ///     (Key::new("quux"), 4_i32),
    /// ]);
    ///
    /// assert_eq!(het_map.get::<i32, _>(&"foo"),  Some(&1_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"bar"),  Some(&2_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"baz"),  Some(&3_i32));
    /// assert_eq!(het_map.get::<i32, _>(&"quux"), Some(&4_i32));
    ///
    /// let occupied_entry = into_vacant(het_map.entry_or_insert_type(Key::new("corge"))).insert_entry(i32::MAX);
    ///
    /// assert_eq!(occupied_entry.key(), &Key::new("corge"));
    /// assert_eq!(occupied_entry.get(), &i32::MAX);
    /// ```
    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, K, T> {
        OccupiedEntry::new(self.inner.insert_entry(value))
    }
}

impl<K, T> fmt::Debug for VacantEntry<'_, K, T>
where
    K: any::Any + fmt::Debug,
    T: any::Any,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}
