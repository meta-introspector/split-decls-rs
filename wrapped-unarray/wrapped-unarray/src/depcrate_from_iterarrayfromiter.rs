// Generated macro for ArrayFromIter (struct)
macro_rules! Depcrate_from_iterArrayFromIter {
() => {
// Module: crate::from_iter
// Provides: {"ArrayFromIter"}
// Dependencies: {}
# [doc = " A wrapper type to collect an [`Iterator`] into an array"] # [doc = ""] # [doc = " ```"] # [doc = " # use unarray::*;"] # [doc = " let iter = vec![1, 2, 3].into_iter();"] # [doc = " let ArrayFromIter(array) = iter.collect();"] # [doc = ""] # [doc = " assert_eq!(array, Some([1, 2, 3]));"] # [doc = " ```"] # [doc = " Since iterators don't carry compile-time information about their length (even"] # [doc = " [`core::iter::ExactSizeIterator`] only provides this at runtime), collecting may fail if the"] # [doc = " iterator doesn't yield **exactly** `N` elements:"] # [doc = " ```"] # [doc = " use unarray::*;"] # [doc = " let too_many = vec![1, 2, 3, 4].into_iter();"] # [doc = " let ArrayFromIter::<i32, 3>(option) = too_many.collect();"] # [doc = " assert!(option.is_none());"] # [doc = ""] # [doc = " let too_few = vec![1, 2].into_iter();"] # [doc = " let ArrayFromIter::<i32, 3>(option) = too_few.collect();"] # [doc = " assert!(option.is_none());"] # [doc = " ```"] pub struct ArrayFromIter < T , const N : usize > (pub Option < [T ; N] >) ;
};
}
