use heterogeneous_hash_map::{
    HeterogeneousHashMap,
    Key,
};

use alloc_crate::string::String;

#[derive(Clone, Debug, PartialEq, Eq)]
struct AlchemyItem {
    name: String,
    quantity: u32,
}

impl AlchemyItem {
    fn new(name: &str, quantity: u32) -> Self {
        Self {
            name: String::from(name),
            quantity,
        }
    }
}

impl Default for AlchemyItem {
    fn default() -> Self {
        Self::new("Alcohest", u32::MAX)
    }
}

struct NonExistentItemCategory;

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_insert_entry1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let occupied_entry = entry.insert_entry(AlchemyItem::new("Swallow", 3_u32));

        assert_eq!(occupied_entry.key(), &Key::new(1_usize));
        assert_eq!(occupied_entry.get(), &AlchemyItem::new("Swallow", 3_u32));
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(1));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(1_usize), &AlchemyItem::new("Swallow", 3_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(1_usize), &AlchemyItem::new("Swallow", 3_u32))));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_insert_entry2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);

    {
        let entry = het_map.entry_or_insert_type::<AlchemyItem>(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let occupied_entry = entry.insert_entry(AlchemyItem::new("Cat", 1_u32));

        assert_eq!(occupied_entry.key(), &Key::new(3_usize));
        assert_eq!(occupied_entry.get(), &AlchemyItem::new("Cat", 1_u32));
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Cat", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(3_usize), &AlchemyItem::new("Cat", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_insert_entry3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), AlchemyItem::new("Cat", 1_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let occupied_entry = entry.insert_entry(AlchemyItem::new("Golden Oriole", 10_u32));

        assert_eq!(occupied_entry.key(), &Key::new(2_usize));
        assert_eq!(occupied_entry.get(), &AlchemyItem::new("Golden Oriole", 10_u32));
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 10_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 10_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 10_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = AlchemyItem::new("Swallow", 3_u32);
        let result = entry.or_insert(AlchemyItem::new("Swallow", 3_u32));

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(1));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(1_usize), &AlchemyItem::new("Swallow", 3_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = AlchemyItem::new("Cat", 1_u32);
        let result = entry.or_insert(AlchemyItem::new("Cat", 1_u32));

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Cat", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(3_usize), &AlchemyItem::new("Cat", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), AlchemyItem::new("Cat", 1_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem ,_>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = AlchemyItem::new("Golden Oriole", 1_u32);
        let result = entry.or_insert(AlchemyItem::new("Golden Oriole", 10_u32));

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem ,_>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert_with1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = AlchemyItem::new("Specter Oil", 7_u32);
        let func = || { expected.clone() };
        let result = entry.or_insert_with(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(1));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Specter Oil", 7_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(1_usize), &AlchemyItem::new("Specter Oil", 7_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Specter Oil", 7_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert_with2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = AlchemyItem::new("Specter Oil", 7_u32);
        let func = || { expected.clone() };
        let result = entry.or_insert_with(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Specter Oil", 7_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(3_usize), &AlchemyItem::new("Specter Oil", 7_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Specter Oil", 7_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert_with3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), AlchemyItem::new("Cat", 1_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = AlchemyItem::new("Golden Oriole", 1_u32);
        let func = || { AlchemyItem::new("Specter Oil", 7_u32) };
        let result = entry.or_insert_with(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert_with_key1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = AlchemyItem::new("Specter Oil", 7_u32);
        let func = |key: &Key<usize, AlchemyItem>| {
            if key == &Key::new(1_usize) {
                AlchemyItem::new("Specter Oil", 7_u32)
            } else {
                AlchemyItem::new("Vampire Oil", 17_u32)
            }
        };
        let result = entry.or_insert_with_key(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(1));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Specter Oil", 7_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(1_usize), &AlchemyItem::new("Specter Oil", 7_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Specter Oil", 7_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert_with_key2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = AlchemyItem::new("Vampire Oil", 17_u32);
        let func = |key: &Key<usize, AlchemyItem>| {
            if key == &Key::new(1_usize) {
                AlchemyItem::new("Specter Oil", 7_u32)
            } else {
                AlchemyItem::new("Vampire Oil", 17_u32)
            }
        };
        let result = entry.or_insert_with_key(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Vampire Oil", 17_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(3_usize), &AlchemyItem::new("Vampire Oil", 17_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Vampire Oil", 17_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_insert_with_key3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), AlchemyItem::new("Cat", 1_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = AlchemyItem::new("Golden Oriole", 1_u32);
        let func = |key: &Key<usize, AlchemyItem>| {
            if key == &Key::new(1_usize) {
                AlchemyItem::new("Specter Oil", 7_u32)
            } else {
                AlchemyItem::new("Vampire Oil", 17_u32)
            }
        };
        let result = entry.or_insert_with_key(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_and_modify1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let func = |value: &mut AlchemyItem| {
            value.quantity += 1;
        };
        let new_entry = entry.and_modify(func);

        assert_eq!(new_entry.key(), &Key::new(1_usize));
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_and_modify2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let func = |value: &mut AlchemyItem| {
            value.quantity += 1;
        };
        let new_entry = entry.and_modify(func);

        assert_eq!(new_entry.key(), &Key::new(3_usize));
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_and_modify3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), AlchemyItem::new("Cat", 1_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let func = |value: &mut AlchemyItem| {
            value.quantity += 1;
        };
        let new_entry = entry.and_modify(func);

        assert_eq!(new_entry.key(), &Key::new(2_usize));
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 2_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 2_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 2_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_default1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), None);

    {
        let entry = het_map.entry_or_insert_type::<AlchemyItem>(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = AlchemyItem::default();
        let result = entry.or_default();

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(1));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::default()));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(1_usize), &AlchemyItem::default())));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::default()));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_default2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(2));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), None);
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), None);

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), None);

    {
        let entry = het_map.entry_or_insert_type::<AlchemyItem>(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = AlchemyItem::default();
        let result = entry.or_default();

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::default()));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(3_usize), &AlchemyItem::default())));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::default()));
}

#[rustfmt::skip]
#[test]
fn test_heterogeneous_hash_map_or_default3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<AlchemyItem>();
    het_map.extend([
        (Key::new(1_usize), AlchemyItem::new("Swallow", 3_u32)),
        (Key::new(2_usize), AlchemyItem::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), AlchemyItem::new("Cat", 1_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));

    {
        let entry = het_map.entry_or_insert_type::<AlchemyItem>(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = AlchemyItem::new("Golden Oriole", 1_u32);
        let result = entry.or_default();

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<AlchemyItem>(), Some(3));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<AlchemyItem, _>(&key));
    assert_eq!(het_map.get::<AlchemyItem, _>(&key), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<AlchemyItem, _>(&key), Some((&Key::new(2_usize), &AlchemyItem::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(1_usize)), Some(&AlchemyItem::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(2_usize)), Some(&AlchemyItem::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<AlchemyItem, _>(&Key::new(3_usize)), Some(&AlchemyItem::new("Cat", 1_u32)));
}
