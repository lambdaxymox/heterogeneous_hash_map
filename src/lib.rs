#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![no_std]
extern crate std;

use core::any;
use core::fmt;
use core::iter;
use core::ops;
use core::borrow::Borrow;

use std::collections::HashMap;
use std::any::TypeId;
use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

#[derive(Debug)]
pub struct Key<T> {
    id: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Key<T> {
    #[inline]
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub const fn id(self) -> usize {
        self.id
    }
}

impl<T> From<usize> for Key<T> {
    fn from(id: usize) -> Self {
        Self::new(id)
    }
}

impl<T> Default for Key<T> {
    fn default() -> Self {
        Self::new(usize::default())
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<T> Copy for Key<T> {}

impl<T> hash::Hash for Key<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Key<T> {}

impl<T> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl<T> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> fmt::Display for Key<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{id}", id = self.id)
    }
}

pub struct Iter<'a, T> {
    iter: opaque::index_map::map::Iter<'a, Key<T>, T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(iter: opaque::index_map::map::Iter<'a, Key<T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a Key<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> iter::FusedIterator for Iter<'_, T> {}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { iter: self.iter.clone() }
    }
}

impl<'a, T> Default for Iter<'a, T> {
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

pub struct IterMut<'a, T> {
    iter: opaque::index_map::map::IterMut<'a, Key<T>, T>,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    const fn new(iter: opaque::index_map::map::IterMut<'a, Key<T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (&'a Key<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T> iter::FusedIterator for IterMut<'a, T> {}

impl<'a, T> fmt::Debug for IterMut<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<'a, T> Default for IterMut<'a, T> {
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

pub struct Keys<'a, T> {
    iter: opaque::index_map::map::Keys<'a, Key<T>, T>,
}

impl<'a, T> Keys<'a, T> {
    fn new(iter: opaque::index_map::map::Keys<'a, Key<T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, T> Clone for Keys<'a, T> {
    fn clone(&self) -> Self {
        Keys { iter: self.iter.clone() }
    }
}

impl<'a, T> Iterator for Keys<'a, T> {
    type Item = &'a Key<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> DoubleEndedIterator for Keys<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, T> ExactSizeIterator for Keys<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T> iter::FusedIterator for Keys<'a, T> {}

impl<'a, T> fmt::Debug for Keys<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<'a, T> Default for Keys<'a, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct Values<'a, T> {
    iter: opaque::index_map::map::Values<'a, Key<T>, T>,
}

impl<'a, T> Values<'a, T> {
    #[inline]
    const fn new(iter: opaque::index_map::map::Values<'a, Key<T>, T>) -> Self {
        Self { iter }
    }
}

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> DoubleEndedIterator for Values<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<'a, T> ExactSizeIterator for Values<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T> iter::FusedIterator for Values<'a, T> {}

impl<'a, T> Clone for Values<'a, T> {
    fn clone(&self) -> Self {
        Values { iter: self.iter.clone() }
    }
}

impl<'a, T> fmt::Debug for Values<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, T> Default for Values<'a, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[repr(transparent)]
pub struct Map<T>
where
    T: any::Any,
{
    inner: opaque::index_map::TypeProjectedIndexMap<Key<T>, T>,
}

impl<T> Map<T>
where
    T: any::Any,
{
    fn from_inner(map: &opaque::index_map::TypeProjectedIndexMap<Key<T>, T>) -> &Self {
        unsafe { &*(map as *const opaque::index_map::TypeProjectedIndexMap<Key<T>, T> as *const Self) }
    }

    fn from_inner_mut(map: &mut opaque::index_map::TypeProjectedIndexMap<Key<T>, T>) -> &mut Self {
        unsafe { &mut *(map as *const opaque::index_map::TypeProjectedIndexMap<Key<T>, T> as *mut Self) }
    }
}

impl<T> Map<T>
where
    T: any::Any,
{
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> Map<T>
where
    T: any::Any,
{
    pub fn contains_key(&self, key: &Key<T>) -> bool {
        self.inner.contains_key(key)
    }

    pub fn get(&self, key: &Key<T>) -> Option<&T> {
        self.inner.get(key)
    }

    pub fn get_key_value(&self, key: &Key<T>) -> Option<(&Key<T>, &T)> {
        self.inner.get_key_value(key)
    }

    pub fn get_mut(&mut self, key: &Key<T>) -> Option<&mut T> {
        self.inner.get_mut(key)
    }

    pub fn get_disjoint_mut<Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&'_ mut T>; N]
    where
        Key<T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        self.inner.get_disjoint_mut(ks)
    }

    pub fn insert(&mut self, key: Key<T>, value: T) -> Option<T> {
        self.inner.insert(key, value)
    }

    pub fn remove(&mut self, key: &Key<T>) -> Option<T> {
        self.inner.swap_remove(key)
    }

    pub fn remove_entry(&mut self, key: &Key<T>) -> Option<(Key<T>, T)> {
        self.inner.swap_remove_entry(key)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self.inner.iter_mut())
    }

    pub fn keys(&self) -> Keys<T> {
        Keys::new(self.inner.keys())
    }

    pub fn values(&self) -> Values<T> {
        Values::new(self.inner.values())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&Key<T>, &mut T) -> bool,
    {
        self.inner.retain(keep)
    }
}

impl<T> PartialEq for Map<T>
where
    T: any::Any + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.inner, &other.inner)
    }
}

impl<T> Eq for Map<T>
where
    T: any::Any + Eq,
{
}

impl<T> fmt::Debug for Map<T>
where
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<T> ops::Index<&Key<T>> for Map<T>
where
    T: any::Any,
{
    type Output = T;

    fn index(&self, key: &Key<T>) -> &Self::Output {
        &self.inner[key]
    }
}

pub struct HeterogeneousHashMap {
    map: HashMap<TypeId, opaque::index_map::TypeErasedIndexMap>,
}

impl HeterogeneousHashMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl HeterogeneousHashMap {
    pub fn insert_type<T>(&mut self)
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();
        let map = opaque::index_map::TypeErasedIndexMap::new::<Key<T>, T>();

        self.map.insert(type_id, map);
    }

    pub fn contains_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();

        self.map.contains_key(&type_id)
    }

    pub fn get_map_unchecked<T>(&self) -> &Map<T>
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();
        let map = self.map[&type_id].as_proj::<Key<T>, T, hash::RandomState, alloc::Global>();

        Map::from_inner(map)
    }

    pub fn get_map<T>(&self) -> Option<&Map<T>>
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            return None;
        }

        let map = self.map
            .get(&type_id)
            .map(|m| m.as_proj::<Key<T>, T, hash::RandomState, alloc::Global>())?;

        Some(Map::from_inner(map))
    }

    pub fn get_map_mut<T>(&mut self) -> Option<&mut Map<T>>
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            return None;
        }

        let map = self.map
            .get_mut(&type_id)
            .map(|m| m.as_proj_mut::<Key<T>, T, hash::RandomState, alloc::Global>())?;

        Some(Map::from_inner_mut(map))
    }

    pub fn get_or_insert_map_mut<T>(&mut self) -> &mut Map<T>
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();
        if !self.map.contains_key(&type_id) {
            self.insert_type::<T>();
        }

        self.get_map_mut::<T>().unwrap()
    }
}

impl HeterogeneousHashMap {
    pub fn contains_key<T>(&self, key: &Key<T>) -> bool
    where
        T: any::Any,
    {
        let type_id = TypeId::of::<T>();
        match self.map.get(&type_id) {
            Some(opaque_map) => {
                let proj_map = opaque_map.as_proj::<Key<T>, T, hash::RandomState, alloc::Global>();
                proj_map.contains_key(key)
            }
            None => false,
        }
    }

    pub fn get<T>(&self, key: &Key<T>) -> Option<&T>
    where
        T: any::Any,
    {
        let map = self.get_map::<T>()?;

        map.get(key)
    }

    pub fn get_key_value<T>(&self, key: &Key<T>) -> Option<(&Key<T>, &T)>
    where
        T: any::Any,
    {
        let map = self.get_map::<T>()?;

        map.get_key_value(key)
    }

    pub fn get_mut<T>(&mut self, key: &Key<T>) -> Option<&mut T>
    where
        T: any::Any,
    {
        let map = self.get_map_mut::<T>()?;

        map.get_mut(key)
    }

    pub fn get_disjoint_mut<T, Q, const N: usize>(&mut self, ks: [&Q; N]) -> Option<[Option<&'_ mut T>; N]>
    where
        T: any::Any,
        Key<T>: Borrow<Q>,
        Q: any::Any + hash::Hash + Eq + ?Sized,
    {
        let map = self.get_map_mut::<T>()?;

        Some(map.get_disjoint_mut(ks))
    }

    pub fn insert<T>(&mut self, key: Key<T>, value: T) -> Option<T>
    where
        T: any::Any,
    {
        let map = self.get_or_insert_map_mut::<T>();

        map.insert(key, value)
    }

    pub fn remove<T>(&mut self, key: &Key<T>) -> Option<T>
    where
        T: any::Any,
    {
        let map = self.get_map_mut::<T>()?;

        map.remove(key)
    }

    pub fn remove_entry<T>(&mut self, key: &Key<T>) -> Option<(Key<T>, T)>
    where
        T: any::Any,
    {
        let map = self.get_map_mut::<T>()?;

        map.remove_entry(key)
    }
}

impl fmt::Debug for HeterogeneousHashMap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("HeterogeneousHashMap").finish()
    }
}

