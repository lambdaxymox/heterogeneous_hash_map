use heterogeneous_hash_map::{
    HomogeneousHashMap,
    Key,
};

use std::vec::Vec;

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_len1() {
    let map: HomogeneousHashMap<u64, i64> = HomogeneousHashMap::new();

    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_is_empty1() {
    let map: HomogeneousHashMap<u64, i64> = HomogeneousHashMap::new();

    assert!(map.is_empty());
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_contains_no_values1() {
    let map: HomogeneousHashMap<u64, i64> = HomogeneousHashMap::new();
    for key in 0..65536 {
        assert!(!map.contains_key(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_get1() {
    let map: HomogeneousHashMap<u64, i64> = HomogeneousHashMap::new();
    for key in 0..65536 {
        let result = map.get(&key);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_len2() {
    let map: HomogeneousHashMap<usize, i64> = HomogeneousHashMap::new();

    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_is_empty2() {
    let map: HomogeneousHashMap<usize, i64> = HomogeneousHashMap::new();

    assert!(map.is_empty());
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_contains_no_values2() {
    let map: HomogeneousHashMap<usize, i64> = HomogeneousHashMap::new();
    for key in 0..65536 {
        assert!(!map.contains_key(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_empty_get2() {
    let map: HomogeneousHashMap<usize, i64> = HomogeneousHashMap::new();
    for key in 0..65536 {
        let result = map.get(&key);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get1() {
    let mut map = HomogeneousHashMap::new();

    assert_eq!(map.get(&"a"), None);
    assert_eq!(map.get(&"b"), None);
    assert_eq!(map.get(&"c"), None);
    assert_eq!(map.get(&"d"), None);

    map.insert(Key::new("a"), 1_i32);
    map.insert(Key::new("b"), 2_i32);
    map.insert(Key::new("c"), 3_i32);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), Some(&3));
    assert_eq!(map.get(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get2() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(0_usize), 1_i32),
        (Key::new(1_usize), 2_i32),
        (Key::new(2_usize), 3_i32),
        (Key::new(3_usize), 4_i32),
        (Key::new(4_usize), 5_i32),
        (Key::new(5_usize), 6_i32),
    ]);

    assert_eq!(map.get(&0_usize), Some(&1_i32));
    assert_eq!(map.get(&1_usize), Some(&2_i32));
    assert_eq!(map.get(&2_usize), Some(&3_i32));
    assert_eq!(map.get(&3_usize), Some(&4_i32));
    assert_eq!(map.get(&4_usize), Some(&5_i32));
    assert_eq!(map.get(&5_usize), Some(&6_i32));
    assert_eq!(map.get(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get3() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("a"), 1_i32),
        (Key::new("b"), 2_i32),
        (Key::new("c"), 3_i32),
    ]);

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"c"), Some(&3_i32));
    assert_eq!(map.get(&"b"), Some(&2_i32));

    map.remove(&"b");

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"c"), Some(&3_i32));
    assert_eq!(map.get(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get4() {
    let mut map: HomogeneousHashMap<char, ()> = ('a'..='z').map(|c| (Key::new(c), ())).collect();
    assert_eq!(map.get(&'*'), None);

    map.insert(Key::new('*'), ());
    assert_eq!(map.get(&'*'), Some(&()));

    map.insert(Key::new('a'), ());
    assert_eq!(map.get(&'a'), Some(&()));
    assert_eq!(map.get(&'*'), Some(&()));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get6() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("a"), 1_i32),
        (Key::new("b"), 2_i32),
    ]);

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"b"), Some(&2_i32));
    assert_eq!(map.get(&"c"), None);

    map.insert(Key::new("c"), 3_i32);

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"b"), Some(&2_i32));
    assert_eq!(map.get(&"c"), Some(&3_i32));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_key_value1() {
    let mut map = HomogeneousHashMap::new();

    assert_eq!(map.get_key_value(&"a"), None);
    assert_eq!(map.get_key_value(&"b"), None);
    assert_eq!(map.get_key_value(&"c"), None);
    assert_eq!(map.get_key_value(&"d"), None);

    map.insert(Key::new("a"), 1_i32);
    map.insert(Key::new("b"), 2_i32);
    map.insert(Key::new("c"), 3_i32);

    assert_eq!(map.get_key_value(&"a"), Some((&Key::new("a"), &1_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&Key::new("b"), &2_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&Key::new("c"), &3_i32)));
    assert_eq!(map.get_key_value(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_key_value2() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(0_usize), 1_i32),
        (Key::new(1_usize), 2_i32),
        (Key::new(2_usize), 3_i32),
        (Key::new(3_usize), 4_i32),
        (Key::new(4_usize), 5_i32),
        (Key::new(5_usize), 6_i32),
    ]);

    assert_eq!(map.get_key_value(&0_usize), Some((&Key::new(0_usize), &1_i32)));
    assert_eq!(map.get_key_value(&1_usize), Some((&Key::new(1_usize), &2_i32)));
    assert_eq!(map.get_key_value(&2_usize), Some((&Key::new(2_usize), &3_i32)));
    assert_eq!(map.get_key_value(&3_usize), Some((&Key::new(3_usize), &4_i32)));
    assert_eq!(map.get_key_value(&4_usize), Some((&Key::new(4_usize), &5_i32)));
    assert_eq!(map.get_key_value(&5_usize), Some((&Key::new(5_usize), &6_i32)));
    assert_eq!(map.get_key_value(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_key_value3() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("a"), 1_i32),
        (Key::new("b"), 2_i32),
        (Key::new("c"), 3_i32),
    ]);

    assert_eq!(map.get_key_value(&"a"), Some((&Key::new("a"), &1_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&Key::new("c"), &3_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&Key::new("b"), &2_i32)));

    map.remove(&"b");

    assert_eq!(map.get_key_value(&"a"), Some((&Key::new("a"), &1_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&Key::new("c"), &3_i32)));
    assert_eq!(map.get_key_value(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_key_value4() {
    let mut map: HomogeneousHashMap<char, ()> = ('a'..='z').map(|c| (Key::new(c), ())).collect();
    assert_eq!(map.get_key_value(&'*'), None);

    map.insert(Key::new('*'), ());
    assert_eq!(map.get_key_value(&'*'), Some((&Key::new('*'), &())));

    map.insert(Key::new('a'), ());
    assert_eq!(map.get_key_value(&'a'), Some((&Key::new('a'), &())));
    assert_eq!(map.get_key_value(&'*'), Some((&Key::new('*'), &())));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_key_value5() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("a"), 1_i32),
        (Key::new("b"), 2_i32),
    ]);

    assert_eq!(map.get_key_value(&"a"), Some((&Key::new("a"), &1_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&Key::new("b"), &2_i32)));
    assert_eq!(map.get_key_value(&"c"), None);

    map.insert(Key::new("c"), 3_i32);

    assert_eq!(map.get_key_value(&"a"), Some((&Key::new("a"), &1_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&Key::new("b"), &2_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&Key::new("c"), &3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut1() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::new();
    let expected = [None, None, None, None, None, None];
    let result = map.get_disjoint_mut([&"1", &"2", &"3", &"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut2() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 20_i32),
        Some(&mut 30_i32),
        Some(&mut 40_i32),
        Some(&mut 50_i32),
        Some(&mut 60_i32),
    ];
    let result = map.get_disjoint_mut([&"1", &"2", &"3", &"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut3() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 20_i32),
        Some(&mut 30_i32),
    ];
    let result = map.get_disjoint_mut([&"1", &"2", &"3"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut4() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 40_i32),
        Some(&mut 50_i32),
        Some(&mut 60_i32),
    ];
    let result = map.get_disjoint_mut([&"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut5() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 30_i32),
        Some(&mut 50_i32),
    ];
    let result = map.get_disjoint_mut([&"1", &"3", &"5"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut6() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 20_i32),
        Some(&mut 40_i32),
        Some(&mut 60_i32),
    ];
    let result = map.get_disjoint_mut([&"2", &"4", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut_partial_success1() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        None,
        Some(&mut 30_i32),
        None,
        Some(&mut 50_i32),
        None,
    ];
    let result = map.get_disjoint_mut([&"1", &"20", &"3", &"40", &"5", &"60"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_get_disjoint_mut_partial_success2() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 20_i32),
        Some(&mut 30_i32),
        None,
        Some(&mut 40_i32),
        Some(&mut 50_i32),
        Some(&mut 60_i32),
        None,
    ];
    let result = map.get_disjoint_mut([&"1", &"2", &"3", &"200", &"4", &"5", &"6", &"100"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_homogeneous_hash_map_get_disjoint_mut_repeat_indices1() {
    let mut map: HomogeneousHashMap<&str, i32> = HomogeneousHashMap::from([
        (Key::new("1"), 10_i32),
        (Key::new("2"), 20_i32),
        (Key::new("3"), 30_i32),
        (Key::new("4"), 40_i32),
        (Key::new("5"), 50_i32),
        (Key::new("6"), 60_i32),
    ]);
    let _ = map.get_disjoint_mut([&"1", &"2", &"2", &"4", &"5", &"6"]);

    assert!(true);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_homogeneous_hash_map_get_disjoint_mut_repeat_indices2() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 20_i32),
        (Key::new(3_usize), 30_i32),
        (Key::new(4_usize), 40_i32),
        (Key::new(5_usize), 50_i32),
        (Key::new(6_usize), 60_i32),
    ]);
    let _ = map.get_disjoint_mut([&1, &1, &1, &2, &2, &3]);

    assert!(true);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_keys1() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 24_i32),
        (Key::new(3_usize), 58_i32),
    ]);
    for key in map.keys() {
        assert!(map.contains_key(key));
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_keys2() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 24_i32),
        (Key::new(3_usize), 58_i32),
    ]);
    let expected = Vec::from([Key::new(1_usize), Key::new(2_usize), Key::new(3_usize)]);
    let result = {
        let mut _result: Vec<Key<usize, i32>> = map.keys().cloned().collect();
        _result.sort();
        _result
    };

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_keys3() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 24_i32),
        (Key::new(3_usize), 58_i32),
    ]);
    let expected = Vec::from([
        Some(Key::new(1_usize)),
        Some(Key::new(2_usize)),
        Some(Key::new(3_usize)),
    ]);
    let mut iter = map.keys();

    assert!(expected.contains(&iter.next().cloned()));
    assert!(expected.contains(&iter.next().cloned()));
    assert!(expected.contains(&iter.next().cloned()));
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_keys4() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 24_i32),
        (Key::new(3_usize), 58_i32),
    ]);
    let expected = Vec::from([Some(10_i32), Some(24_i32), Some(58_i32)]);
    let mut iter = map.keys();

    assert!(expected.contains(&map.get(iter.next().unwrap()).cloned()));
    assert!(expected.contains(&map.get(iter.next().unwrap()).cloned()));
    assert!(expected.contains(&map.get(iter.next().unwrap()).cloned()));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_values1() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 24_i32),
        (Key::new(3_usize), 58_i32),
    ]);
    let expected = Vec::from([10_i32, 24_i32, 58_i32]);
    let result: Vec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_values2() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::new();
    let expected = Vec::new();
    let result: Vec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_values3() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 10_i32),
        (Key::new(2_usize), 24_i32),
        (Key::new(3_usize), 58_i32),
    ]);
    let expected = Vec::from([Some(10_i32), Some(24_i32), Some(58_i32)]);
    let mut iter = map.values();

    assert!(expected.contains(&iter.next().cloned()));
    assert!(expected.contains(&iter.next().cloned()));
    assert!(expected.contains(&iter.next().cloned()));
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_iter1() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);

    for (key, _value) in map.iter() {
        assert!(map.contains_key(key));
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_iter2() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);

    for (key, value) in map.iter() {
        let expected = Some(value);
        let result = map.get(key);

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_iter3() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);
    let expected = Vec::from([
        (Key::new(39_usize), 5_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(81_usize), 36_i32),
        (Key::new(89_usize), 92_i32),
    ]);
    let result = {
        let mut _result: Vec<(Key<usize, i32>, i32)> = map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        _result.sort();
        _result
    };

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_iter4() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);
    let expected = Vec::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);
    let mut iter = map.iter();

    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_iter5() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::new();
    let mut iter = map.iter();

    for _ in 0..65536 {
        assert!(iter.next().is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_into_iter1() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);

    for (key, _value) in map.clone().into_iter() {
        assert!(map.contains_key(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_into_iter2() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);

    for (key, value) in map.clone().into_iter() {
        let expected = Some(&value);
        let result = map.get(&key);

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_into_iter3() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);
    let expected = Vec::from([
        (Key::new(39_usize), 5_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(81_usize), 36_i32),
        (Key::new(89_usize), 92_i32),
    ]);
     let result = {
        let mut _result: Vec<(Key<usize, i32>, i32)> = map
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        _result.sort();
        _result
    };

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_into_iter4() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);
    let expected = Vec::from([
        (Key::new(89_usize), 92_i32),
        (Key::new(40_usize), 59_i32),
        (Key::new(80_usize), 87_i32),
        (Key::new(39_usize), 5_i32),
        (Key::new(62_usize), 11_i32),
        (Key::new(81_usize), 36_i32),
    ]);
    let mut iter = map.into_iter();

    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert!(expected.contains(&iter.next().map(|(k, v)| (k.clone(), v.clone())).unwrap()));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_into_iter5() {
    let map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::new();
    let mut iter = map.into_iter();

    for _ in 0..65536 {
        assert!(iter.next().is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_clear1() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_clear2() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert!(!map.is_empty());
    assert_eq!(map.len(), 6);

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_clear3() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert!(map.contains_key(&1_usize));
    assert!(map.contains_key(&2_usize));
    assert!(map.contains_key(&3_usize));
    assert!(map.contains_key(&4_usize));
    assert!(map.contains_key(&5_usize));
    assert!(map.contains_key(&6_usize));

    map.clear();

    assert!(!map.contains_key(&1_usize));
    assert!(!map.contains_key(&2_usize));
    assert!(!map.contains_key(&3_usize));
    assert!(!map.contains_key(&4_usize));
    assert!(!map.contains_key(&5_usize));
    assert!(!map.contains_key(&6_usize));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove1() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.remove(&1_usize), Some(20_i32));
    assert_eq!(map.remove(&2_usize), Some(2043_i32));
    assert_eq!(map.remove(&3_usize), Some(4904_i32));
    assert_eq!(map.remove(&4_usize), Some(20994_i32));
    assert_eq!(map.remove(&5_usize), Some(302_i32));
    assert_eq!(map.remove(&6_usize), Some(5_i32));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove2() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove3() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove4() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.remove(&6_usize), Some(5_i32));
    assert_eq!(map.remove(&5_usize), Some(302_i32));
    assert_eq!(map.remove(&4_usize), Some(20994_i32));
    assert_eq!(map.remove(&3_usize), Some(4904_i32));
    assert_eq!(map.remove(&2_usize), Some(2043_i32));
    assert_eq!(map.remove(&1_usize), Some(20_i32));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove_entry1() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.remove_entry(&1_usize), Some((Key::new(1_usize), 20_i32)));
    assert_eq!(map.remove_entry(&2_usize), Some((Key::new(2_usize), 2043_i32)));
    assert_eq!(map.remove_entry(&3_usize), Some((Key::new(3_usize), 4904_i32)));
    assert_eq!(map.remove_entry(&4_usize), Some((Key::new(4_usize), 20994_i32)));
    assert_eq!(map.remove_entry(&5_usize), Some((Key::new(5_usize), 302_i32)));
    assert_eq!(map.remove_entry(&6_usize), Some((Key::new(6_usize), 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove_entry2() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove_entry3() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), Some(&5_i32));

    let _ = map.remove_entry(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), Some(&302_i32));
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove_entry(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), Some(&20994_i32));
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove_entry(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), Some(&4904_i32));
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove_entry(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), Some(&2043_i32));
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove_entry(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&Key::new(1_usize)), Some(&20_i32));
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);

    let _ = map.remove_entry(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&Key::new(1_usize)), None);
    assert_eq!(map.get(&Key::new(2_usize)), None);
    assert_eq!(map.get(&Key::new(3_usize)), None);
    assert_eq!(map.get(&Key::new(4_usize)), None);
    assert_eq!(map.get(&Key::new(5_usize)), None);
    assert_eq!(map.get(&Key::new(6_usize)), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_remove_entry4() {
    let mut map: HomogeneousHashMap<usize, i32> = HomogeneousHashMap::from([
        (Key::new(1_usize), 20_i32),
        (Key::new(2_usize), 2043_i32),
        (Key::new(3_usize), 4904_i32),
        (Key::new(4_usize), 20994_i32),
        (Key::new(5_usize), 302_i32),
        (Key::new(6_usize), 5_i32),
    ]);

    assert_eq!(map.remove_entry(&6_usize), Some((Key::new(6_usize), 5_i32)));
    assert_eq!(map.remove_entry(&5_usize), Some((Key::new(5_usize), 302_i32)));
    assert_eq!(map.remove_entry(&4_usize), Some((Key::new(4_usize), 20994_i32)));
    assert_eq!(map.remove_entry(&3_usize), Some((Key::new(3_usize), 4904_i32)));
    assert_eq!(map.remove_entry(&2_usize), Some((Key::new(2_usize), 2043_i32)));
    assert_eq!(map.remove_entry(&1_usize), Some((Key::new(1_usize), 20_i32)));
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_insert1() {
    let mut map = HomogeneousHashMap::new();

    assert_eq!(map.insert(Key::new(1803_usize), 1778_i32), None);
    assert_eq!(map.insert(Key::new(1057_usize), 2437_i32), None);
    assert_eq!(map.insert(Key::new(1924_usize), 185_i32),  None);
    assert_eq!(map.insert(Key::new(302_usize), 2457_i32),  None);
    assert_eq!(map.insert(Key::new(949_usize), 2176_i32),  None);
    assert_eq!(map.insert(Key::new(2968_usize), 1398_i32), None);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_insert2() {
    let mut map = HomogeneousHashMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert(Key::new(1803_usize), 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_unchecked(&Key::new(1803_usize)), &1778_i32);

    let _ = map.insert(Key::new(1057_usize), 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get_unchecked(&Key::new(1803_usize)), &1778_i32);
    assert_eq!(map.get_unchecked(&Key::new(1057_usize)), &2437_i32);

    let _ = map.insert(Key::new(1924_usize), 185_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get_unchecked(&Key::new(1803_usize)), &1778_i32);
    assert_eq!(map.get_unchecked(&Key::new(1057_usize)), &2437_i32);
    assert_eq!(map.get_unchecked(&Key::new(1924_usize)), &185_i32);

    let _ = map.insert(Key::new(302_usize), 2457_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.get_unchecked(&Key::new(1803_usize)), &1778_i32);
    assert_eq!(map.get_unchecked(&Key::new(1057_usize)), &2437_i32);
    assert_eq!(map.get_unchecked(&Key::new(1924_usize)), &185_i32);
    assert_eq!(map.get_unchecked(&Key::new(302_usize)),  &2457_i32);

    let _ = map.insert(Key::new(949_usize), 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.get_unchecked(&Key::new(1803_usize)), &1778_i32);
    assert_eq!(map.get_unchecked(&Key::new(1057_usize)), &2437_i32);
    assert_eq!(map.get_unchecked(&Key::new(1924_usize)), &185_i32);
    assert_eq!(map.get_unchecked(&Key::new(302_usize)),  &2457_i32);
    assert_eq!(map.get_unchecked(&Key::new(949_usize)),  &2176_i32);

    let _ = map.insert(Key::new(2968_usize), 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.get_unchecked(&Key::new(1803_usize)), &1778_i32);
    assert_eq!(map.get_unchecked(&Key::new(1057_usize)), &2437_i32);
    assert_eq!(map.get_unchecked(&Key::new(1924_usize)), &185_i32);
    assert_eq!(map.get_unchecked(&Key::new(302_usize)),  &2457_i32);
    assert_eq!(map.get_unchecked(&Key::new(949_usize)),  &2176_i32);
    assert_eq!(map.get_unchecked(&Key::new(2968_usize)), &1398_i32);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_retain1() {
    let mut map: HomogeneousHashMap<usize, ()> = HomogeneousHashMap::from([
        (Key::new(344_usize),  ()),
        (Key::new(1646_usize), ()),
        (Key::new(2371_usize), ()),
        (Key::new(52_usize),   ()),
        (Key::new(789_usize),  ()),
        (Key::new(1205_usize), ()),
        (Key::new(28_usize),   ()),
        (Key::new(136_usize),  ()),
    ]);
    let expected = map.clone();
    map.retain(|_k, _v| true);

    assert_eq!(map.len(), 8);
    assert_eq!(map, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_retain2() {
    let mut map: HomogeneousHashMap<usize, ()> = HomogeneousHashMap::from([
        (Key::new(344_usize),  ()),
        (Key::new(1646_usize), ()),
        (Key::new(2371_usize), ()),
        (Key::new(52_usize),   ()),
        (Key::new(789_usize),  ()),
        (Key::new(1205_usize), ()),
        (Key::new(28_usize),   ()),
        (Key::new(136_usize),  ()),
    ]);
    let expected = HomogeneousHashMap::new();
    map.retain(|_k, _v| false);

    assert_eq!(map.len(), 0);
    assert_eq!(map, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_retain3() {
    let mut map: HomogeneousHashMap<usize, ()> = HomogeneousHashMap::from([
        (Key::new(344_usize),  ()),
        (Key::new(1646_usize), ()),
        (Key::new(2371_usize), ()),
        (Key::new(52_usize),   ()),
        (Key::new(789_usize),  ()),
        (Key::new(1205_usize), ()),
        (Key::new(28_usize),   ()),
        (Key::new(136_usize),  ()),
    ]);
    let expected = HomogeneousHashMap::from([
        (Key::new(344_usize),  ()),
        (Key::new(1646_usize), ()),
        (Key::new(52_usize),   ()),
        (Key::new(28_usize),   ()),
        (Key::new(136_usize),  ()),
    ]);
    map.retain(|k, _v| k.id() % 2 == 0);

    assert_eq!(map.len(), 5);
    assert_eq!(map, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_retain4() {
    let mut map: HomogeneousHashMap<usize, ()> = HomogeneousHashMap::from([
        (Key::new(344_usize),  ()),
        (Key::new(1646_usize), ()),
        (Key::new(2371_usize), ()),
        (Key::new(52_usize),   ()),
        (Key::new(789_usize),  ()),
        (Key::new(1205_usize), ()),
        (Key::new(28_usize),   ()),
        (Key::new(136_usize),  ()),
    ]);
    let expected = HomogeneousHashMap::from([
        (Key::new(2371_usize), ()),
        (Key::new(789_usize),  ()),
        (Key::new(1205_usize), ()),
    ]);
    map.retain(|k, _v| k.id() % 2 != 0);

    assert_eq!(map.len(), 3);
    assert_eq!(map, expected);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_reserve1() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve(additional);

    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_reserve2() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve(additional);

    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert(Key::new(0_usize), usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert(Key::new(i), 0_usize);
    }

    map.insert(Key::new(map.capacity() - 1), usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map[&Key::new(0_usize)], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map[&Key::new(i)], 0_usize);
    }
    assert_eq!(map[&Key::new(map.len() - 1)], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_reserve3() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..4 {
        let old_capacity = map.capacity();
        map.reserve(additional);

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert(Key::new(length), usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert(Key::new(j), i);
        }
        map.insert(Key::new(map.capacity() - 1), usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..map.len() {
            if map[&Key::new(j)] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map[&Key::new(current_start)], usize::MAX);
        for k in (current_start + 1)..current_end {
            let value = map[&Key::new(k)];
            assert_eq!(value, i);
        }
        assert_eq!(map[&Key::new(current_end)], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_try_reserve1() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve(additional), Ok(()));
    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_try_reserve2() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve(additional), Ok(()));
    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert(Key::new(0_usize), usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert(Key::new(i), 0_usize);
    }

    map.insert(Key::new(map.capacity() - 1), usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map[&Key::new(0_usize)], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map[&Key::new(i)], 0_usize);
    }
    assert_eq!(map[&Key::new(map.len() - 1)], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_try_reserve3() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..4 {
        let old_capacity = map.capacity();
        assert_eq!(map.try_reserve(additional), Ok(()));

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert(Key::new(length), usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert(Key::new(j), i);
        }
        map.insert(Key::new(map.capacity() - 1), usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..map.len() {
            if map[&Key::new(j)] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map[&Key::new(current_start)], usize::MAX);
        for k in (current_start + 1)..current_end {
            let value = map[&Key::new(k)];
            assert_eq!(value, i);
        }
        assert_eq!(map[&Key::new(current_end)], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_shrink_to_fit1() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::with_capacity(10);

    assert_eq!(map.capacity(), 10);

    map.extend([
        (Key::new(1_usize), usize::MAX),
        (Key::new(2_usize), usize::MAX),
        (Key::new(3_usize), usize::MAX),
    ]);

    assert!(map.len() <= map.capacity());
    map.shrink_to_fit();
    assert_eq!(map.len(), map.capacity());
}

#[rustfmt::skip]
#[test]
fn test_homogeneous_hash_map_shrink_to_fit2() {
    let mut map: HomogeneousHashMap<usize, usize> = HomogeneousHashMap::new();
    for i in 0..128 {
        assert_eq!(map.len(), i);

        map.insert(Key::new(i), i * i);

        assert_eq!(map.len(), i + 1);
        assert!(map.capacity() >= i + 1);
        assert_eq!(map[&Key::new(i)], i * i);
        assert_eq!(map.get(&i), Some(&(i * i)));

        map.shrink_to_fit();

        assert_eq!(map.len(), i + 1);
        assert_eq!(map.capacity(), i + 1);
        assert_eq!(map[&Key::new(i)], i * i);
        assert_eq!(map.get(&i), Some(&(i * i)));
    }
}
