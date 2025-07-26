use heterogeneous_hash_map::{Key, HeterogeneousHashMap};

use alloc_crate::string::String;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Potion {
    name: String,
    quantity: u32,
}

impl Potion {
    fn new(name: &str, quantity: u32) -> Self {
        Self {
            name: String::from(name),
            quantity,
        }
    }
}

impl Default for Potion {
    fn default() -> Self {
        Self::new("Alcohest", u32::MAX)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Oil {
    name: String,
    quantity: u32,
}

impl Oil {
    fn new(name: &str, quantity: u32) -> Self {
        Self {
            name: String::from(name),
            quantity,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QuestItem {
    name: String,
    quantity: u32,
}

impl QuestItem {
    fn new(name: &str, quantity: u32) -> Self {
        Self {
            name: String::from(name),
            quantity,
        }
    }
}

struct NonExistentItemCategory;

#[test]
fn test_heterogeneous_hash_map_insert_entry1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let occupied_entry = entry.insert_entry(Potion::new("Swallow", 3_u32));

        assert_eq!(occupied_entry.key(), &Key::new(1_usize));
        assert_eq!(occupied_entry.get(), &Potion::new("Swallow", 3_u32));
    }

    assert_eq!(het_map.len::<Potion>(), Some(1));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(1_usize), &Potion::new("Swallow", 3_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_insert_entry2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type::<Potion>(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let occupied_entry = entry.insert_entry(Potion::new("Cat", 1_u32));

        assert_eq!(occupied_entry.key(), &Key::new(3_usize));
        assert_eq!(occupied_entry.get(), &Potion::new("Cat", 1_u32));
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Cat", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(3_usize), &Potion::new("Cat", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_insert_entry3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), Potion::new("Cat", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let occupied_entry = entry.insert_entry(Potion::new("Golden Oriole", 10_u32));

        assert_eq!(occupied_entry.key(), &Key::new(2_usize));
        assert_eq!(occupied_entry.get(), &Potion::new("Golden Oriole", 10_u32));
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 10_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 10_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 10_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = Potion::new("Swallow", 3_u32);
        let result = entry.or_insert(Potion::new("Swallow", 3_u32));

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(1));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(1_usize), &Potion::new("Swallow", 3_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = Potion::new("Cat", 1_u32);
        let result = entry.or_insert(Potion::new("Cat", 1_u32));

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Cat", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(3_usize), &Potion::new("Cat", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), Potion::new("Cat", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = Potion::new("Golden Oriole", 1_u32);
        let result = entry.or_insert(Potion::new("Golden Oriole", 10_u32));

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert_with1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = Potion::new("Specter Oil", 7_u32);
        let func = || { expected.clone() };
        let result = entry.or_insert_with(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(1));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Specter Oil", 7_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(1_usize), &Potion::new("Specter Oil", 7_u32))));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert_with2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = Potion::new("Specter Oil", 7_u32);
        let func = || { expected.clone() };
        let result = entry.or_insert_with(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Specter Oil", 7_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(3_usize), &Potion::new("Specter Oil", 7_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Specter Oil", 7_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert_with3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), Potion::new("Cat", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = Potion::new("Golden Oriole", 1_u32);
        let func = || { Potion::new("Specter Oil", 7_u32) };
        let result = entry.or_insert_with(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert_with_key1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = Potion::new("Specter Oil", 7_u32);
        let func = |key: &Key<usize, Potion>| {
            if key == &Key::new(1_usize) {
                Potion::new("Specter Oil", 7_u32)
            } else {
                Potion::new("Vampire Oil", 17_u32)
            }
        };
        let result = entry.or_insert_with_key(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(1));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Specter Oil", 7_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(1_usize), &Potion::new("Specter Oil", 7_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Specter Oil", 7_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert_with_key2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = Potion::new("Vampire Oil", 17_u32);
        let func = |key: &Key<usize, Potion>| {
            if key == &Key::new(1_usize) {
                Potion::new("Specter Oil", 7_u32)
            } else {
                Potion::new("Vampire Oil", 17_u32)
            }
        };
        let result = entry.or_insert_with_key(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Vampire Oil", 17_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(3_usize), &Potion::new("Vampire Oil", 17_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Vampire Oil", 17_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_insert_with_key3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), Potion::new("Cat", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = Potion::new("Golden Oriole", 1_u32);
        let func = |key: &Key<usize, Potion>| {
            if key == &Key::new(1_usize) {
                Potion::new("Specter Oil", 7_u32)
            } else {
                Potion::new("Vampire Oil", 17_u32)
            }
        };
        let result = entry.or_insert_with_key(func);

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_and_modify1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let func = |value: &mut Potion| {
            value.quantity += 1;
        };
        let new_entry = entry.and_modify(func);

        assert_eq!(new_entry.key(), &Key::new(1_usize));
    }

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_and_modify2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let func = |value: &mut Potion| {
            value.quantity += 1;
        };
        let new_entry = entry.and_modify(func);

        assert_eq!(new_entry.key(), &Key::new(3_usize));
    }

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_and_modify3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), Potion::new("Cat", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let func = |value: &mut Potion| {
            value.quantity += 1;
        };
        let new_entry = entry.and_modify(func);

        assert_eq!(new_entry.key(), &Key::new(2_usize));
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 2_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 2_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 2_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_default1() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(1_usize);

    assert_eq!(het_map.len::<Potion>(), Some(0));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), None);

    {
        let entry = het_map.entry_or_insert_type::<Potion>(Key::new(1_usize));

        assert_eq!(entry.key(), &Key::new(1_usize));

        let expected = Potion::default();
        let result = entry.or_default();

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(1));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::default()));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(1_usize), &Potion::default())));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::default()));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_default2() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(3_usize);

    assert_eq!(het_map.len::<Potion>(), Some(2));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(!het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), None);
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), None);

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), None);

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type::<Potion>(Key::new(3_usize));

        assert_eq!(entry.key(), &Key::new(3_usize));

        let expected = Potion::default();
        let result = entry.or_default();

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::default()));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(3_usize), &Potion::default())));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::default()));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}

#[test]
fn test_heterogeneous_hash_map_or_default3() {
    let mut het_map: HeterogeneousHashMap<usize> = HeterogeneousHashMap::new();
    het_map.insert_type::<Potion>();
    het_map.insert_type::<Oil>();
    het_map.insert_type::<QuestItem>();
    het_map.extend([
        (Key::new(1_usize), Potion::new("Swallow", 3_u32)),
        (Key::new(2_usize), Potion::new("Golden Oriole", 1_u32)),
        (Key::new(3_usize), Potion::new("Cat", 1_u32)),
    ]);
    het_map.extend([
        (Key::new(1_usize), Oil::new("Specter Oil", 2_u32)),
        (Key::new(2_usize), Oil::new("Vampire Oil", 1_u32)),
        (Key::new(3_usize), Oil::new("Relict Oil", 5_u32)),
        (Key::new(4_usize), Oil::new("Hanged Man's Venom", 8_u32)),
    ]);
    let key = Key::new(2_usize);

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));

    {
        let entry = het_map.entry_or_insert_type::<Potion>(Key::new(2_usize));

        assert_eq!(entry.key(), &Key::new(2_usize));

        let expected = Potion::new("Golden Oriole", 1_u32);
        let result = entry.or_default();

        assert_eq!(result, &expected);
    }

    assert_eq!(het_map.len::<Potion>(), Some(3));
    assert_eq!(het_map.len::<Oil>(), Some(4));
    assert_eq!(het_map.len::<QuestItem>(), Some(0));
    assert_eq!(het_map.len::<NonExistentItemCategory>(), None);

    assert!(het_map.contains_key::<Potion, _>(&key));
    assert_eq!(het_map.get::<Potion, _>(&key), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get_key_value::<Potion, _>(&key), Some((&Key::new(2_usize), &Potion::new("Golden Oriole", 1_u32))));

    assert_eq!(het_map.get::<Potion, _>(&Key::new(1_usize)), Some(&Potion::new("Swallow", 3_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(2_usize)), Some(&Potion::new("Golden Oriole", 1_u32)));
    assert_eq!(het_map.get::<Potion, _>(&Key::new(3_usize)), Some(&Potion::new("Cat", 1_u32)));

    assert_eq!(het_map.get::<Oil, _>(&Key::new(1_usize)), Some(&Oil::new("Specter Oil", 2_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(2_usize)), Some(&Oil::new("Vampire Oil", 1_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(3_usize)), Some(&Oil::new("Relict Oil", 5_u32)));
    assert_eq!(het_map.get::<Oil, _>(&Key::new(4_usize)), Some(&Oil::new("Hanged Man's Venom", 8_u32)));
}
