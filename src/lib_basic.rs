#![feature(test)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use slab::Slab;



extern crate test;
use test::Bencher;

pub mod movies;

#[derive(Debug)]
struct SlabEntry<T> {
    val: T,
    ref1: i32,
    ref2: String,
}

#[derive(Debug)]
struct MultiIndexMap<T> {
    slab: Slab<SlabEntry<T>>,
    map1: HashMap<i32, usize>,
    map2: HashMap<String, usize>,
}

impl<T> MultiIndexMap<T> {
    fn new() -> MultiIndexMap<T> {
        return MultiIndexMap {slab: Slab::new(), map1: HashMap::new(), map2: HashMap::new()};
    }

    fn try_insert(&mut self, value: T, key1: i32, key2: String) -> bool{
        let entry1 = self.map1.entry(key1.clone());
        let entry2 = self.map2.entry(key2.clone());
        let result = match entry1 {
            Entry::Occupied(_entry1) => {
                //println!("key not unique in map 1");
                false
            },
            Entry::Vacant(entry1) => {
                let result = match entry2 {
                    Entry::Occupied(_entry2) => {
                        //println!("key not unique in map 2");
                        false
                    },
                    Entry::Vacant(entry2) => {
                        let entry = self.slab.vacant_entry();
                        entry1.insert(entry.key());
                        entry2.insert(entry.key());
                        entry.insert(SlabEntry::<T>{val: value, ref1: key1, ref2: key2});
                        true
                    }
                };
                result
            }
        };

        return result;
    }

    fn try_delete_1(&mut self, key1: i32) -> bool{
        let entry1 = self.map1.remove_entry(&key1);
        let result = match entry1 {
            Some((_key, val)) => {
                //println!("key found in map 1");

                let entry = &self.slab[val];
                self.map2.remove(&entry.ref2);
                self.slab.remove(val);
                true
            },
            None => {
                //println!("key not found in map 1");
                false
            }
        };
        return result;
    }
}

fn comment_out() -> bool{
    return true;
    // #[derive(Debug)]
    // struct SlabEntry<'a, T> {
    //     val: T,
    //     ref1: Entry<'a, i32, usize>,
    //     ref2: Entry<'a, String, usize>,
    // }

    // #[derive(Debug)]
    // struct MultiIndexMap<'a, T> {
    //     slab: Slab<SlabEntry<'a, T>>,
    //     map1: HashMap<i32, usize>,
    //     map2: HashMap<String, usize>,
    // }

    // impl<'a, T> MultiIndexMap<'a, T> {
    //     fn new() -> MultiIndexMap<'a, T> {
    //         return MultiIndexMap {slab: Slab::new(), map1: HashMap::new(), map2: HashMap::new()};
    //     }

    //     fn try_insert(&'a mut self, value: T, key1: i32, key2: String) -> bool{
    //         let entry1 = self.map1.entry(key1.clone());
    //         let entry2 = self.map2.entry(key2.clone());
    //         let result = match entry1 {
    //             Entry::Occupied(_entry1) => {
    //                 println!("key not unique in map 1");
    //                 false
    //             },
    //             Entry::Vacant(entry1) => {
    //                 let result = match entry2 {
    //                     Entry::Occupied(_entry2) => {
    //                         println!("key not unique in map 2");
    //                         false
    //                     },
    //                     Entry::Vacant(entry2) => {
    //                         let entry = self.slab.vacant_entry();
    //                         entry1.insert(entry.key());
    //                         entry2.insert(entry.key());
    //                         let entry1 = self.map1.entry(key1);
    //                         let entry2 = self.map2.entry(key2);
    //                         entry.insert(SlabEntry::<'a, T>{val: value, ref1: entry1, ref2: entry2});//
    //                         true
    //                     }
    //                 };
    //                 result
    //             }
    //         };

    //         return result;
    //     }
    // }

    // pub fn add(left: usize, right: usize) -> usize {
    //     movies::play("Test in add".to_string());
        
    //     left + right
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_instantiation() {
        let _default = MultiIndexMap::<u32>::new();
    }

    #[test]
    fn insert() {
        let mut default = MultiIndexMap::<u32>::new();
        assert_eq!(default.try_insert(1, -1, "A".to_string()), true);
        assert_eq!(default.try_insert(2, -2, "B".to_string()), true);
    }

    #[test]
    fn delete() {
        let mut default = MultiIndexMap::<u32>::new();
        assert_eq!(default.try_insert(1, -1, "A".to_string()), true);
        assert_eq!(default.try_insert(2, -2, "B".to_string()), true);
        assert_eq!(default.slab.len(), 2);
        assert_eq!(default.try_delete_1(-1), true);
        assert_eq!(default.slab.len(), 1);
        assert_eq!(default.try_delete_1(-3), false);
        assert_eq!(default.try_delete_1(-2), true);
    }

    fn add_and_remove_n(n: u32) {
        let mut default = MultiIndexMap::<u32>::new();
        for x in 1..n {
            default.try_insert(x,x as i32,x.to_string());
        }
        for x in 1..n {
            default.try_delete_1(x as i32);
        }
    }

    fn add_and_remove_n_comparison(n: u32) {
        let mut slab = Slab::<SlabEntry::<u32>>::new();
        let mut map1 = HashMap::<i32, usize>::new();
        let mut map2 = HashMap::<String, usize>::new();

        for x in 1..n {
            let idx = slab.insert(SlabEntry::<u32>{val: x, ref1: x as i32, ref2: x.to_string()});
            map1.insert(x as i32, idx);
            map2.insert(x.to_string(), idx);
        }
        for x in 1..n {
            map2.remove(&x.to_string());
            let idx = map1.remove(&(x as i32)).unwrap();
            slab.remove(idx);
        }
    }

    #[bench]
    fn bench_add_and_remove_10_3(b: &mut Bencher) {
        b.iter(|| add_and_remove_n(test::black_box(1000)));
    }
    #[bench]
    fn bench_add_and_remove_10_4(b: &mut Bencher) {
        b.iter(|| add_and_remove_n(test::black_box(10000)));
    }
    #[bench]
    fn bench_add_and_remove_10_5(b: &mut Bencher) {
        b.iter(|| add_and_remove_n(test::black_box(100000)));
    }

    //#[bench]
    //fn bench_add_and_remove_n_comparison(b: &mut Bencher) {
    //    b.iter(|| add_and_remove_n_comparison(test::black_box(1000)));
    //}
}
