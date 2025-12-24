use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod std_tests {
    #![allow(clippy::reversed_empty_ranges)]
    use super::*;
    use crate::alloc::{format, string::{String, ToString}};
    use core::mem::size_of;
    use core::usize;
    struct DropCounter<'a> {
        count: &'a mut u32,
    }
    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            *self.count += 1;
        }
    }
    #[test]
    fn test_small_vec_struct() {
        assert!(size_of::< ThinVec < u8 >> () == size_of::< usize > ());
    }
    #[test]
    fn test_double_drop() {
        struct TwoVec<T> {
            x: ThinVec<T>,
            y: ThinVec<T>,
        }
        let (mut count_x, mut count_y) = (0, 0);
        {
            let mut tv = TwoVec {
                x: ThinVec::new(),
                y: ThinVec::new(),
            };
            tv.x.push(DropCounter { count: &mut count_x });
            tv.y.push(DropCounter { count: &mut count_y });
            drop(tv.x);
        }
        assert_eq!(count_x, 1);
        assert_eq!(count_y, 1);
    }
    #[test]
    fn test_reserve() {
        let mut v = ThinVec::new();
        assert_eq!(v.capacity(), 0);
        v.reserve(2);
        assert!(v.capacity() >= 2);
        for i in 0..16 {
            v.push(i);
        }
        assert!(v.capacity() >= 16);
        v.reserve(16);
        assert!(v.capacity() >= 32);
        v.push(16);
        v.reserve(16);
        assert!(v.capacity() >= 33)
    }
    #[test]
    fn test_extend() {
        let mut v = ThinVec::<usize>::new();
        let mut w = ThinVec::new();
        v.extend(w.clone());
        assert_eq!(v, & []);
        v.extend(0..3);
        for i in 0..3 {
            w.push(i)
        }
        assert_eq!(v, w);
        v.extend(3..10);
        for i in 3..10 {
            w.push(i)
        }
        assert_eq!(v, w);
        v.extend(w.clone());
        assert!(v.iter().eq(w.iter().chain(w.iter())));
        #[derive(PartialEq, Debug)]
        struct Foo;
        let mut a = ThinVec::new();
        let b = thin_vec![Foo, Foo];
        a.extend(b);
        assert_eq!(a, & [Foo, Foo]);
        let mut count_x = 0;
        {
            let mut x = ThinVec::new();
            let y = thin_vec![DropCounter { count : & mut count_x }];
            x.extend(y);
        }
        assert_eq!(count_x, 1);
    }
    #[test]
    fn test_slice_from_mut() {
        let mut values = thin_vec![1, 2, 3, 4, 5];
        {
            let slice = &mut values[2..];
            assert!(slice == [3, 4, 5]);
            for p in slice {
                *p += 2;
            }
        }
        assert!(values == [1, 2, 5, 6, 7]);
    }
    #[test]
    fn test_slice_to_mut() {
        let mut values = thin_vec![1, 2, 3, 4, 5];
        {
            let slice = &mut values[..2];
            assert!(slice == [1, 2]);
            for p in slice {
                *p += 1;
            }
        }
        assert!(values == [2, 3, 3, 4, 5]);
    }
    #[test]
    fn test_split_at_mut() {
        let mut values = thin_vec![1, 2, 3, 4, 5];
        {
            let (left, right) = values.split_at_mut(2);
            {
                let left: &[_] = left;
                assert!(left[..left.len()] == [1, 2]);
            }
            for p in left {
                *p += 1;
            }
            {
                let right: &[_] = right;
                assert!(right[..right.len()] == [3, 4, 5]);
            }
            for p in right {
                *p += 2;
            }
        }
        assert_eq!(values, [2, 3, 5, 6, 7]);
    }
    #[test]
    fn test_clone() {
        let v: ThinVec<i32> = thin_vec![];
        let w = thin_vec![1, 2, 3];
        assert_eq!(v, v.clone());
        let z = w.clone();
        assert_eq!(w, z);
        assert!(w.as_ptr() != z.as_ptr())
    }
    #[test]
    fn test_clone_from() {
        let mut v = thin_vec![];
        let three: ThinVec<Box<_>> = thin_vec![Box::new(1), Box::new(2), Box::new(3)];
        let two: ThinVec<Box<_>> = thin_vec![Box::new(4), Box::new(5)];
        v.clone_from(&three);
        assert_eq!(v, three);
        v.clone_from(&three);
        assert_eq!(v, three);
        v.clone_from(&two);
        assert_eq!(v, two);
        v.clone_from(&three);
        assert_eq!(v, three)
    }
    #[test]
    fn test_retain() {
        let mut vec = thin_vec![1, 2, 3, 4];
        vec.retain(|&x| x % 2 == 0);
        assert_eq!(vec, [2, 4]);
    }
    #[test]
    fn test_retain_mut() {
        let mut vec = thin_vec![9, 9, 9, 9];
        let mut i = 0;
        vec.retain_mut(|x| {
            i += 1;
            *x = i;
            i != 4
        });
        assert_eq!(vec, [1, 2, 3]);
    }
    #[test]
    fn test_dedup() {
        fn case(a: ThinVec<i32>, b: ThinVec<i32>) {
            let mut v = a;
            v.dedup();
            assert_eq!(v, b);
        }
        case(thin_vec![], thin_vec![]);
        case(thin_vec![1], thin_vec![1]);
        case(thin_vec![1, 1], thin_vec![1]);
        case(thin_vec![1, 2, 3], thin_vec![1, 2, 3]);
        case(thin_vec![1, 1, 2, 3], thin_vec![1, 2, 3]);
        case(thin_vec![1, 2, 2, 3], thin_vec![1, 2, 3]);
        case(thin_vec![1, 2, 3, 3], thin_vec![1, 2, 3]);
        case(thin_vec![1, 1, 2, 2, 2, 3, 3], thin_vec![1, 2, 3]);
    }
    #[test]
    fn test_dedup_by_key() {
        fn case(a: ThinVec<i32>, b: ThinVec<i32>) {
            let mut v = a;
            v.dedup_by_key(|i| *i / 10);
            assert_eq!(v, b);
        }
        case(thin_vec![], thin_vec![]);
        case(thin_vec![10], thin_vec![10]);
        case(thin_vec![10, 11], thin_vec![10]);
        case(thin_vec![10, 20, 30], thin_vec![10, 20, 30]);
        case(thin_vec![10, 11, 20, 30], thin_vec![10, 20, 30]);
        case(thin_vec![10, 20, 21, 30], thin_vec![10, 20, 30]);
        case(thin_vec![10, 20, 30, 31], thin_vec![10, 20, 30]);
        case(thin_vec![10, 11, 20, 21, 22, 30, 31], thin_vec![10, 20, 30]);
    }
    #[test]
    fn test_dedup_by() {
        let mut vec = thin_vec!["foo", "bar", "Bar", "baz", "bar"];
        vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
        assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
        let mut vec = thin_vec![
            ("foo", 1), ("foo", 2), ("bar", 3), ("bar", 4), ("bar", 5)
        ];
        vec.dedup_by(|a, b| {
            a.0 == b.0
                && {
                    b.1 += a.1;
                    true
                }
        });
        assert_eq!(vec, [("foo", 3), ("bar", 12)]);
    }
    #[test]
    fn test_dedup_unique() {
        let mut v0: ThinVec<Box<_>> = thin_vec![
            Box::new(1), Box::new(1), Box::new(2), Box::new(3)
        ];
        v0.dedup();
        let mut v1: ThinVec<Box<_>> = thin_vec![
            Box::new(1), Box::new(2), Box::new(2), Box::new(3)
        ];
        v1.dedup();
        let mut v2: ThinVec<Box<_>> = thin_vec![
            Box::new(1), Box::new(2), Box::new(3), Box::new(3)
        ];
        v2.dedup();
    }
    #[test]
    fn zero_sized_values() {
        let mut v = ThinVec::new();
        assert_eq!(v.len(), 0);
        v.push(());
        assert_eq!(v.len(), 1);
        v.push(());
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), None);
        assert_eq!(v.iter().count(), 0);
        v.push(());
        assert_eq!(v.iter().count(), 1);
        v.push(());
        assert_eq!(v.iter().count(), 2);
        for &() in &v {}
        assert_eq!(v.iter_mut().count(), 2);
        v.push(());
        assert_eq!(v.iter_mut().count(), 3);
        v.push(());
        assert_eq!(v.iter_mut().count(), 4);
        for &mut () in &mut v {}
        unsafe {
            v.set_len(0);
        }
        assert_eq!(v.iter_mut().count(), 0);
    }
    #[test]
    fn test_partition() {
        assert_eq!(
            thin_vec![] .into_iter().partition(| x : & i32 | * x < 3), (thin_vec![],
            thin_vec![])
        );
        assert_eq!(
            thin_vec![1, 2, 3] .into_iter().partition(| x | * x < 4), (thin_vec![1, 2,
            3], thin_vec![])
        );
        assert_eq!(
            thin_vec![1, 2, 3] .into_iter().partition(| x | * x < 2), (thin_vec![1],
            thin_vec![2, 3])
        );
        assert_eq!(
            thin_vec![1, 2, 3] .into_iter().partition(| x | * x < 0), (thin_vec![],
            thin_vec![1, 2, 3])
        );
    }
    #[test]
    fn test_zip_unzip() {
        let z1 = thin_vec![(1, 4), (2, 5), (3, 6)];
        let (left, right): (ThinVec<_>, ThinVec<_>) = z1.iter().cloned().unzip();
        assert_eq!((1, 4), (left[0], right[0]));
        assert_eq!((2, 5), (left[1], right[1]));
        assert_eq!((3, 6), (left[2], right[2]));
    }
    #[test]
    fn test_vec_truncate_drop() {
        static mut DROPS: u32 = 0;
        struct Elem(i32);
        impl Drop for Elem {
            fn drop(&mut self) {
                unsafe {
                    DROPS += 1;
                }
            }
        }
        let mut v = thin_vec![Elem(1), Elem(2), Elem(3), Elem(4), Elem(5)];
        assert_eq!(unsafe { DROPS }, 0);
        v.truncate(3);
        assert_eq!(unsafe { DROPS }, 2);
        v.truncate(0);
        assert_eq!(unsafe { DROPS }, 5);
    }
    #[test]
    #[should_panic]
    fn test_vec_truncate_fail() {
        struct BadElem(i32);
        impl Drop for BadElem {
            fn drop(&mut self) {
                let BadElem(ref mut x) = *self;
                if *x == 0xbadbeef {
                    panic!("BadElem panic: 0xbadbeef")
                }
            }
        }
        let mut v = thin_vec![BadElem(1), BadElem(2), BadElem(0xbadbeef), BadElem(4)];
        v.truncate(0);
    }
    #[test]
    fn test_index() {
        let vec = thin_vec![1, 2, 3];
        assert!(vec[1] == 2);
    }
    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let vec = thin_vec![1, 2, 3];
        let _ = vec[3];
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_1() {
        let x = thin_vec![1, 2, 3, 4, 5];
        let _ = &x[!0..];
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_2() {
        let x = thin_vec![1, 2, 3, 4, 5];
        let _ = &x[..6];
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_3() {
        let x = thin_vec![1, 2, 3, 4, 5];
        let _ = &x[!0..4];
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_4() {
        let x = thin_vec![1, 2, 3, 4, 5];
        let _ = &x[1..6];
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_5() {
        let x = thin_vec![1, 2, 3, 4, 5];
        let _ = &x[3..2];
    }
    #[test]
    #[should_panic]
    fn test_swap_remove_empty() {
        let mut vec = ThinVec::<i32>::new();
        vec.swap_remove(0);
    }
    #[test]
    fn test_move_items() {
        let vec = thin_vec![1, 2, 3];
        let mut vec2 = thin_vec![];
        for i in vec {
            vec2.push(i);
        }
        assert_eq!(vec2, [1, 2, 3]);
    }
    #[test]
    fn test_move_items_reverse() {
        let vec = thin_vec![1, 2, 3];
        let mut vec2 = thin_vec![];
        for i in vec.into_iter().rev() {
            vec2.push(i);
        }
        assert_eq!(vec2, [3, 2, 1]);
    }
    #[test]
    fn test_move_items_zero_sized() {
        let vec = thin_vec![(), (), ()];
        let mut vec2 = thin_vec![];
        for i in vec {
            vec2.push(i);
        }
        assert_eq!(vec2, [(), (), ()]);
    }
    #[test]
    fn test_drain_items() {
        let mut vec = thin_vec![1, 2, 3];
        let mut vec2 = thin_vec![];
        for i in vec.drain(..) {
            vec2.push(i);
        }
        assert_eq!(vec, []);
        assert_eq!(vec2, [1, 2, 3]);
    }
    #[test]
    fn test_drain_items_reverse() {
        let mut vec = thin_vec![1, 2, 3];
        let mut vec2 = thin_vec![];
        for i in vec.drain(..).rev() {
            vec2.push(i);
        }
        assert_eq!(vec, []);
        assert_eq!(vec2, [3, 2, 1]);
    }
    #[test]
    fn test_drain_items_zero_sized() {
        let mut vec = thin_vec![(), (), ()];
        let mut vec2 = thin_vec![];
        for i in vec.drain(..) {
            vec2.push(i);
        }
        assert_eq!(vec, []);
        assert_eq!(vec2, [(), (), ()]);
    }
    #[test]
    #[should_panic]
    fn test_drain_out_of_bounds() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        v.drain(5..6);
    }
    #[test]
    fn test_drain_range() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        for _ in v.drain(4..) {}
        assert_eq!(v, & [1, 2, 3, 4]);
        let mut v: ThinVec<_> = (1..6).map(|x| x.to_string()).collect();
        for _ in v.drain(1..4) {}
        assert_eq!(v, & [1.to_string(), 5.to_string()]);
        let mut v: ThinVec<_> = (1..6).map(|x| x.to_string()).collect();
        for _ in v.drain(1..4).rev() {}
        assert_eq!(v, & [1.to_string(), 5.to_string()]);
        let mut v: ThinVec<_> = thin_vec![(); 5];
        for _ in v.drain(1..4).rev() {}
        assert_eq!(v, & [(), ()]);
    }
    #[test]
    fn test_drain_inclusive_range() {
        let mut v = thin_vec!['a', 'b', 'c', 'd', 'e'];
        for _ in v.drain(1..=3) {}
        assert_eq!(v, & ['a', 'e']);
        let mut v: ThinVec<_> = (0..=5).map(|x| x.to_string()).collect();
        for _ in v.drain(1..=5) {}
        assert_eq!(v, & ["0".to_string()]);
        let mut v: ThinVec<String> = (0..=5).map(|x| x.to_string()).collect();
        for _ in v.drain(0..=5) {}
        assert_eq!(v, ThinVec::< String >::new());
        let mut v: ThinVec<_> = (0..=5).map(|x| x.to_string()).collect();
        for _ in v.drain(0..=3) {}
        assert_eq!(v, & ["4".to_string(), "5".to_string()]);
        let mut v: ThinVec<_> = (0..=1).map(|x| x.to_string()).collect();
        for _ in v.drain(..=0) {}
        assert_eq!(v, & ["1".to_string()]);
    }
    #[test]
    #[cfg(not(feature = "gecko-ffi"))]
    fn test_drain_max_vec_size() {
        let mut v = ThinVec::<()>::with_capacity(usize::max_value());
        unsafe {
            v.set_len(usize::max_value());
        }
        for _ in v.drain(usize::max_value() - 1..) {}
        assert_eq!(v.len(), usize::max_value() - 1);
        let mut v = ThinVec::<()>::with_capacity(usize::max_value());
        unsafe {
            v.set_len(usize::max_value());
        }
        for _ in v.drain(usize::max_value() - 1..=usize::max_value() - 1) {}
        assert_eq!(v.len(), usize::max_value() - 1);
    }
    #[test]
    #[should_panic]
    fn test_drain_inclusive_out_of_bounds() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        v.drain(5..=5);
    }
    #[test]
    fn test_splice() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        let a = [10, 11, 12];
        v.splice(2..4, a.iter().cloned());
        assert_eq!(v, & [1, 2, 10, 11, 12, 5]);
        v.splice(1..3, Some(20));
        assert_eq!(v, & [1, 20, 11, 12, 5]);
    }
    #[test]
    fn test_splice_inclusive_range() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        let a = [10, 11, 12];
        let t1: ThinVec<_> = v.splice(2..=3, a.iter().cloned()).collect();
        assert_eq!(v, & [1, 2, 10, 11, 12, 5]);
        assert_eq!(t1, & [3, 4]);
        let t2: ThinVec<_> = v.splice(1..=2, Some(20)).collect();
        assert_eq!(v, & [1, 20, 11, 12, 5]);
        assert_eq!(t2, & [2, 10]);
    }
    #[test]
    #[should_panic]
    fn test_splice_out_of_bounds() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        let a = [10, 11, 12];
        v.splice(5..6, a.iter().cloned());
    }
    #[test]
    #[should_panic]
    fn test_splice_inclusive_out_of_bounds() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        let a = [10, 11, 12];
        v.splice(5..=5, a.iter().cloned());
    }
    #[test]
    fn test_splice_items_zero_sized() {
        let mut vec = thin_vec![(), (), ()];
        let vec2 = thin_vec![];
        let t: ThinVec<_> = vec.splice(1..2, vec2.iter().cloned()).collect();
        assert_eq!(vec, & [(), ()]);
        assert_eq!(t, & [()]);
    }
    #[test]
    fn test_splice_unbounded() {
        let mut vec = thin_vec![1, 2, 3, 4, 5];
        let t: ThinVec<_> = vec.splice(.., None).collect();
        assert_eq!(vec, & []);
        assert_eq!(t, & [1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_splice_forget() {
        let mut v = thin_vec![1, 2, 3, 4, 5];
        let a = [10, 11, 12];
        ::core::mem::forget(v.splice(2..4, a.iter().cloned()));
        assert_eq!(v, & [1, 2]);
    }
    #[test]
    fn test_splice_from_empty() {
        let mut v = thin_vec![];
        let a = [10, 11, 12];
        v.splice(.., a.iter().cloned());
        assert_eq!(v, & [10, 11, 12]);
    }
    #[test]
    fn test_append() {
        let mut vec = thin_vec![1, 2, 3];
        let mut vec2 = thin_vec![4, 5, 6];
        vec.append(&mut vec2);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
        assert_eq!(vec2, []);
    }
    #[test]
    fn test_split_off() {
        let mut vec = thin_vec![1, 2, 3, 4, 5, 6];
        let vec2 = vec.split_off(4);
        assert_eq!(vec, [1, 2, 3, 4]);
        assert_eq!(vec2, [5, 6]);
    }
    #[test]
    fn test_into_iter_as_slice() {
        let vec = thin_vec!['a', 'b', 'c'];
        let mut into_iter = vec.into_iter();
        assert_eq!(into_iter.as_slice(), & ['a', 'b', 'c']);
        let _ = into_iter.next().unwrap();
        assert_eq!(into_iter.as_slice(), & ['b', 'c']);
        let _ = into_iter.next().unwrap();
        let _ = into_iter.next().unwrap();
        assert_eq!(into_iter.as_slice(), & []);
    }
    #[test]
    fn test_into_iter_as_mut_slice() {
        let vec = thin_vec!['a', 'b', 'c'];
        let mut into_iter = vec.into_iter();
        assert_eq!(into_iter.as_slice(), & ['a', 'b', 'c']);
        into_iter.as_mut_slice()[0] = 'x';
        into_iter.as_mut_slice()[1] = 'y';
        assert_eq!(into_iter.next().unwrap(), 'x');
        assert_eq!(into_iter.as_slice(), & ['y', 'c']);
    }
    #[test]
    fn test_into_iter_debug() {
        let vec = thin_vec!['a', 'b', 'c'];
        let into_iter = vec.into_iter();
        let debug = format!("{:?}", into_iter);
        assert_eq!(debug, "IntoIter(['a', 'b', 'c'])");
    }
    #[test]
    fn test_into_iter_count() {
        assert_eq!(thin_vec![1, 2, 3] .into_iter().count(), 3);
    }
    #[test]
    fn test_into_iter_clone() {
        fn iter_equal<I: Iterator<Item = i32>>(it: I, slice: &[i32]) {
            let v: ThinVec<i32> = it.collect();
            assert_eq!(& v[..], slice);
        }
        let mut it = thin_vec![1, 2, 3].into_iter();
        iter_equal(it.clone(), &[1, 2, 3]);
        assert_eq!(it.next(), Some(1));
        let mut it = it.rev();
        iter_equal(it.clone(), &[3, 2]);
        assert_eq!(it.next(), Some(3));
        iter_equal(it.clone(), &[2]);
        assert_eq!(it.next(), Some(2));
        iter_equal(it.clone(), &[]);
        assert_eq!(it.next(), None);
    }
    #[allow(dead_code)]
    fn assert_covariance() {
        fn drain<'new>(d: Drain<'static, &'static str>) -> Drain<'new, &'new str> {
            d
        }
        fn into_iter<'new>(i: IntoIter<&'static str>) -> IntoIter<&'new str> {
            i
        }
    }
    #[test]
    #[cfg_attr(feature = "gecko-ffi", ignore)]
    fn overaligned_allocations() {
        #[repr(align(256))]
        struct Foo(usize);
        let mut v = thin_vec![Foo(273)];
        for i in 0..0x1000 {
            v.reserve_exact(i);
            assert!(v[0].0 == 273);
            assert!(v.as_ptr() as usize & 0xff == 0);
            v.shrink_to_fit();
            assert!(v[0].0 == 273);
            assert!(v.as_ptr() as usize & 0xff == 0);
        }
    }
    #[test]
    fn test_reserve_exact() {
        let mut v = ThinVec::new();
        assert_eq!(v.capacity(), 0);
        v.reserve_exact(2);
        assert!(v.capacity() >= 2);
        for i in 0..16 {
            v.push(i);
        }
        assert!(v.capacity() >= 16);
        v.reserve_exact(16);
        assert!(v.capacity() >= 32);
        v.push(16);
        v.reserve_exact(16);
        assert!(v.capacity() >= 33)
    }
    #[cfg(feature = "gecko-ffi")]
    #[test]
    fn auto_t_array_basic() {
        crate::auto_thin_vec!(let t : [u8; 10]);
        assert_eq!(t.capacity(), 10);
        assert!(t.is_auto_array());
        assert!(t.uses_stack_allocated_buffer());
        assert!(! t.has_allocation());
        {
            let inner = unsafe { &mut *t.as_mut().as_mut_ptr() };
            for i in 0..30 {
                inner.push(i as u8);
            }
        }
        assert!(t.is_auto_array());
        assert!(! t.uses_stack_allocated_buffer());
        assert_eq!(t.len(), 30);
        assert!(t.has_allocation());
        assert_eq!(t[5], 5);
        assert_eq!(t[29], 29);
        assert!(t.capacity() >= 30);
        {
            let inner = unsafe { &mut *t.as_mut().as_mut_ptr() };
            inner.truncate(5);
        }
        assert_eq!(t.len(), 5);
        assert!(t.capacity() >= 30);
        assert!(t.has_allocation());
        t.as_mut().shrink_to_fit();
        assert!(! t.has_allocation());
        assert!(t.is_auto_array());
        assert!(t.uses_stack_allocated_buffer());
        assert_eq!(t.capacity(), 10);
    }
    #[test]
    #[cfg_attr(feature = "gecko-ffi", ignore)]
    fn test_header_data() {
        macro_rules! assert_aligned_head_ptr {
            ($typename:ty) => {
                { let v : ThinVec <$typename > = ThinVec::with_capacity(1); let head_ptr
                : * mut $typename = v.data_raw(); assert_eq!(head_ptr as usize %
                core::mem::align_of::<$typename > (), 0,
                "expected Header::data<{}> to be aligned", stringify!($typename)); }
            };
        }
        const HEADER_SIZE: usize = core::mem::size_of::<Header>();
        assert_eq!(2 * core::mem::size_of::< usize > (), HEADER_SIZE);
        #[repr(C, align(128))]
        struct Funky<T>(T);
        assert_eq!(padding::< Funky < () >> (), 128 - HEADER_SIZE);
        assert_aligned_head_ptr!(Funky < () >);
        assert_eq!(padding::< Funky < u8 >> (), 128 - HEADER_SIZE);
        assert_aligned_head_ptr!(Funky < u8 >);
        assert_eq!(padding::< Funky < [(); 1024] >> (), 128 - HEADER_SIZE);
        assert_aligned_head_ptr!(Funky < [(); 1024] >);
        assert_eq!(padding::< Funky < [* mut usize; 1024] >> (), 128 - HEADER_SIZE);
        assert_aligned_head_ptr!(Funky < [* mut usize; 1024] >);
    }
    #[cfg(feature = "serde")]
    use serde_test::{assert_tokens, Token};
    #[test]
    #[cfg(feature = "serde")]
    fn test_ser_de_empty() {
        let vec = ThinVec::<u32>::new();
        assert_tokens(&vec, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
    }
    #[test]
    #[cfg(feature = "serde")]
    fn test_ser_de() {
        let mut vec = ThinVec::<u32>::new();
        vec.push(20);
        vec.push(55);
        vec.push(123);
        assert_tokens(
            &vec,
            &[
                Token::Seq { len: Some(3) },
                Token::U32(20),
                Token::U32(55),
                Token::U32(123),
                Token::SeqEnd,
            ],
        );
    }
    #[test]
    fn test_set_len() {
        let mut vec: ThinVec<u32> = thin_vec![];
        unsafe {
            vec.set_len(0);
        }
    }
    #[test]
    #[should_panic(expected = "invalid set_len(1) on empty ThinVec")]
    fn test_set_len_invalid() {
        let mut vec: ThinVec<u32> = thin_vec![];
        unsafe {
            vec.set_len(1);
        }
    }
    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn test_capacity_overflow_header_too_big() {
        let vec: ThinVec<u8> = ThinVec::with_capacity(isize::MAX as usize - 2);
        assert!(vec.capacity() > 0);
    }
    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn test_capacity_overflow_cap_too_big() {
        let vec: ThinVec<u8> = ThinVec::with_capacity(isize::MAX as usize + 1);
        assert!(vec.capacity() > 0);
    }
    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn test_capacity_overflow_size_mul1() {
        let vec: ThinVec<u16> = ThinVec::with_capacity(isize::MAX as usize + 1);
        assert!(vec.capacity() > 0);
    }
    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn test_capacity_overflow_size_mul2() {
        let vec: ThinVec<u16> = ThinVec::with_capacity(isize::MAX as usize / 2 + 1);
        assert!(vec.capacity() > 0);
    }
    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn test_capacity_overflow_cap_really_isnt_isize() {
        let vec: ThinVec<u8> = ThinVec::with_capacity(isize::MAX as usize);
        assert!(vec.capacity() > 0);
    }
}
