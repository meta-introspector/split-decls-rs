macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! slice_owned {
    () => {
        deps!();
        # [doc = " Makes an [`OwnedSlice`] out of an `owner` and a `slicer` function."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rustc_data_structures::owned_slice::{OwnedSlice, slice_owned};"] # [doc = " let vec = vec![1, 2, 3, 4];"] # [doc = ""] # [doc = " // Identical to slicing via `&v[1..3]` but produces an owned slice"] # [doc = " let slice: OwnedSlice = slice_owned(vec, |v| &v[1..3]);"] # [doc = " assert_eq!(&*slice, [2, 3]);"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rustc_data_structures::owned_slice::{OwnedSlice, slice_owned};"] # [doc = " # use std::ops::Deref;"] # [doc = " let vec = vec![1, 2, 3, 4];"] # [doc = ""] # [doc = " // Identical to slicing via `&v[..]` but produces an owned slice"] # [doc = " let slice: OwnedSlice = slice_owned(vec, Deref::deref);"] # [doc = " assert_eq!(&*slice, [1, 2, 3, 4]);"] # [doc = " ```"] pub fn slice_owned < O , F > (owner : O , slicer : F) -> OwnedSlice where O : Send + Sync + 'static , F : FnOnce (& O) -> & [u8] , { try_slice_owned (owner , | x | Ok :: < _ , ! > (slicer (x))) . into_ok () }
    };
}

slice_owned!()