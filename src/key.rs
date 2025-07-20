use core::cmp;
use core::fmt;
use core::hash;
use core::marker;

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
