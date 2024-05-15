#![feature(test)]

use multiindex_derive::CreateMultiIndexMap;
use multi_index_map::MultiIndexMap;

extern crate test;
use test::Bencher;

#[derive(Debug, CreateMultiIndexMap, PartialEq)]
pub struct MyStruct {
    val: u32,
    #[MultiIndexRef]
    ref1: i32,
    #[MultiIndexRef]
    ref2: String,
    #[MultiIndexRef]
    ref3: i32,
    #[MultiIndexRef]
    ref4: i32,
}

impl MyStruct {
    pub fn new(x: u32) -> MyStruct {
        MyStruct{val: x, ref1: -(x as i32), ref2: x.to_string(), ref3: x as i32, ref4: x as i32}
    }
}

#[derive(MultiIndexMap, Debug)]
#[multi_index_derive(Debug)]
struct Order {
    val: u32,
    #[multi_index(hashed_unique)]
    ref1: i32,
    #[multi_index(hashed_unique)]
    ref2: String,
    #[multi_index(hashed_unique)]
    ref3: i32,
    #[multi_index(hashed_unique)]
    ref4: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_instantiation() {
        let default = MultiIndexMyStructMap::new();
    }

    #[test]
    fn insert() {
        let mut default = MultiIndexMyStructMap::new();
        assert_eq!(default.try_insert(MyStruct::new(1)), Result::Ok(()));
        assert_eq!(default.try_insert(MyStruct::new(2)), Result::Ok(()));
    }

    #[test]
    fn delete() {
        let mut default = MultiIndexMyStructMap::new();
        assert_eq!(default.try_insert(MyStruct::new(1)), Result::Ok(()));
        assert_eq!(default.try_insert(MyStruct::new(2)), Result::Ok(()));
        assert_eq!(default.slab.len(), 2);
        assert_eq!(default.remove_by_ref1(-1), Result::Ok(MyStruct::new(1)));
        assert_eq!(default.slab.len(), 1);
        assert_eq!(default.remove_by_ref1(-3), Result::Err(-3));
        assert_eq!(default.remove_by_ref2("2".to_string()), Result::Ok(MyStruct::new(2)));
    }

    fn add_and_remove_n(n: u32) {
        let mut default = MultiIndexMyStructMap::new();
        for x in 1..n {
            let _ = default.try_insert(MyStruct::new(x));
        }
        for x in 1..n {
            let _ = default.remove_by_ref1(-(x as i32));
        }
    }

    fn add_and_remove_n_other_crate(n: u32) {
        let mut map = MultiIndexOrderMap::default();
        for x in 1..n {
            let _ = map.try_insert(Order{val: x,ref1: x as i32, ref2: x.to_string(), ref3: x as i32, ref4: x as i32});
        }
        for x in 1..n {
            let _ = map.remove_by_ref1(&(x as i32));
        }
    }

    fn add_and_get_n(n: u32) {
        let mut default = MultiIndexMyStructMap::new();
        for x in 1..n {
            let _ = default.try_insert(MyStruct::new(x));
        }
        for x in 1..n {
            let _ = default.get_by_ref1(-(x as i32));
            let _ = default.get_by_ref3((n-x) as i32);
        }
    }

    fn add_and_get_n_other_crate(n: u32) {
        let mut map = MultiIndexOrderMap::default();
        for x in 1..n {
            let _ = map.try_insert(Order{val: x,ref1: x as i32, ref2: x.to_string(), ref3: x as i32, ref4: x as i32});
        }
        for x in 1..n {
            let _ = map.get_by_ref1(&(x as i32));
            let _ = map.get_by_ref3(&((n-x) as i32));
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

    #[bench]
    fn bench_add_and_remove_10_3_other_crate(b: &mut Bencher) {
        b.iter(|| add_and_remove_n_other_crate(test::black_box(1000)));
    }
    #[bench]
    fn bench_add_and_remove_10_4_other_crate(b: &mut Bencher) {
        b.iter(|| add_and_remove_n_other_crate(test::black_box(10000)));
    }
    #[bench]
    fn bench_add_and_remove_10_5_other_crate(b: &mut Bencher) {
        b.iter(|| add_and_remove_n_other_crate(test::black_box(100000)));
    }

    #[bench]
    fn bench_add_and_get_10_3(b: &mut Bencher) {
        b.iter(|| add_and_get_n(test::black_box(1000)));
    }
    #[bench]
    fn bench_add_and_get_10_4(b: &mut Bencher) {
        b.iter(|| add_and_get_n(test::black_box(10000)));
    }
    #[bench]
    fn bench_add_and_get_10_5(b: &mut Bencher) {
        b.iter(|| add_and_get_n(test::black_box(100000)));
    }

    #[bench]
    fn bench_add_and_get_10_3_other_crate(b: &mut Bencher) {
        b.iter(|| add_and_get_n_other_crate(test::black_box(1000)));
    }
    #[bench]
    fn bench_add_and_get_10_4_other_crate(b: &mut Bencher) {
        b.iter(|| add_and_get_n_other_crate(test::black_box(10000)));
    }
    #[bench]
    fn bench_add_and_get_10_5_other_crate(b: &mut Bencher) {
        b.iter(|| add_and_get_n_other_crate(test::black_box(100000)));
    }
}
