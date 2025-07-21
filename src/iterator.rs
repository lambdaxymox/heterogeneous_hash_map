use crate::key::Key;
use crate::metadata::TypeMetadata;

use core::any;
use core::fmt;
use core::iter;

use hashbrown::hash_map;

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
    pub(crate) const fn new(iter: opaque::index_map::map::Iter<'a, Key<K, T>, T>) -> Self {
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
    pub(crate) const fn new(iter: opaque::index_map::map::IterMut<'a, Key<K, T>, T>) -> Self {
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
    pub(crate) const fn new(iter: opaque::index_map::map::Keys<'a, Key<K, T>, T>) -> Self {
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
    pub(crate) const fn new(iter: opaque::index_map::map::Values<'a, Key<K, T>, T>) -> Self {
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
    pub(crate) const fn new(iter: opaque::index_map::map::ValuesMut<'a, Key<K, T>, T>) -> Self {
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
    pub(crate) const fn new(iter: opaque::index_map::map::Drain<'a, Key<K, T>, T>) -> Self {
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
    pub(crate) const fn new(iter: opaque::index_map::map::ExtractIf<'a, Key<K, T>, T, F>) -> ExtractIf<'a, K, T, F> {
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
    pub(crate) const fn new(iter: hash_map::Iter<'a, any::TypeId, TypeMetadata>) -> Self {
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

/// A moving iterator over the entries of a hash map.
///
/// Moving iterators are created by the [`HomogeneousHashMap::into_iter`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
/// #
/// let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
///     (Key::new(0_usize), 1_i32),
///     (Key::new(1_usize), 2_i32),
///     (Key::new(2_usize), 3_i32),
///     (Key::new(3_usize), 4_i32),
/// ]);
/// let expected = Vec::from([
///     (Key::new(0_usize), 1_i32),
///     (Key::new(1_usize), 2_i32),
///     (Key::new(2_usize), 3_i32),
///     (Key::new(3_usize), 4_i32),
/// ]);
/// let result = {
///     let mut _result: Vec<(Key<usize, i32>, i32)> = map.into_iter().collect();
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct IntoIter<K, T>
where
    K: any::Any,
    T: any::Any,
{
    iter: opaque::index_map::map::IntoIter<K, T>,
}

impl<K, T> IntoIter<K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs new a moving iterator..
    #[inline]
    pub(crate) const fn new(iter: opaque::index_map::map::IntoIter<K, T>) -> Self {
        Self { iter }
    }
}

impl<K, T> Iterator for IntoIter<K, T>
where
    K: any::Any,
    T: any::Any,
{
    type Item = (K, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, T> DoubleEndedIterator for IntoIter<K, T>
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

impl<K, T> ExactSizeIterator for IntoIter<K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, T> iter::FusedIterator for IntoIter<K, T>
where
    K: any::Any,
    T: any::Any,
{
}

impl<K, T> fmt::Debug for IntoIter<K, T>
where
    K: any::Any + fmt::Debug,
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<K, T> Default for IntoIter<K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

/// A moving iterator over the keys of the entries of the index map.
///
/// Moving key iterators are created by the [`HomogeneousHashMap::into_keys`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
/// #
/// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
///     (Key::new("foo"),  1_i32),
///     (Key::new("bar"),  2_i32),
///     (Key::new("baz"),  3_i32),
///     (Key::new("quux"), 4_i32),
/// ]);
/// let expected = Vec::from([Key::new("bar"), Key::new("baz"), Key::new("foo"), Key::new("quux")]);
/// let result = {
///     let mut _result: Vec<Key<&str, i32>> = map.into_keys().collect();
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result, expected);
/// ```
pub struct IntoKeys<K, T>
where
    K: any::Any,
    T: any::Any,
{
    iter: opaque::index_map::map::IntoKeys<Key<K, T>, T>,
}

impl<K, T> IntoKeys<K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs a new moving key iterator.
    pub(crate) const fn new(iter: opaque::index_map::map::IntoKeys<Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<K, T> Iterator for IntoKeys<K, T>
where
    K: any::Any,
    T: any::Any,
{
    type Item = Key<K, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, T> DoubleEndedIterator for IntoKeys<K, T>
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

impl<K, T> ExactSizeIterator for IntoKeys<K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, T> iter::FusedIterator for IntoKeys<K, T>
where
    K: any::Any,
    T: any::Any,
{
}

impl<K, T> fmt::Debug for IntoKeys<K, T>
where
    K: any::Any + fmt::Debug,
    T: any::Any,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<K, T> Default for IntoKeys<K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

/// A moving iterator over the keys of the entries of the index map.
///
/// Moving key iterators are created by the [`HomogeneousHashMap::into_keys`] method.
///
/// # Examples
///
/// ```
/// # use heterogeneous_hash_map::{Key, HomogeneousHashMap};
/// #
/// let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
///     (Key::new("foo"),  1_i32),
///     (Key::new("bar"),  2_i32),
///     (Key::new("baz"),  3_i32),
///     (Key::new("quux"), 4_i32),
/// ]);
/// let expected = Vec::from([1_i32, 2_i32, 3_i32, 4_i32]);
/// let result = {
///     let mut _result: Vec<i32> = map.into_values().collect();
///     _result.sort();
///     _result
/// };
///
/// assert_eq!(result, expected);
/// ```
pub struct IntoValues<K, T>
where
    K: any::Any,
    T: any::Any,
{
    iter: opaque::index_map::map::IntoValues<Key<K, T>, T>,
}

impl<K, T> IntoValues<K, T>
where
    K: any::Any,
    T: any::Any,
{
    /// Constructs a new moving key iterator.
    pub(crate) const fn new(iter: opaque::index_map::map::IntoValues<Key<K, T>, T>) -> Self {
        Self { iter }
    }
}

impl<K, T> Iterator for IntoValues<K, T>
where
    K: any::Any,
    T: any::Any,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, T> DoubleEndedIterator for IntoValues<K, T>
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

impl<K, T> ExactSizeIterator for IntoValues<K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, T> iter::FusedIterator for IntoValues<K, T>
where
    K: any::Any,
    T: any::Any,
{
}

impl<K, T> fmt::Debug for IntoValues<K, T>
where
    K: any::Any,
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<K, T> Default for IntoValues<K, T>
where
    K: any::Any,
    T: any::Any,
{
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}
