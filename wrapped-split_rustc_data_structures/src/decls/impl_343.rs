macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl OwnedSlice { # [doc = " Slice this slice by `slicer`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rustc_data_structures::owned_slice::{OwnedSlice, slice_owned};"] # [doc = " let vec = vec![1, 2, 3, 4];"] # [doc = ""] # [doc = " // Identical to slicing via `&v[1..3]` but produces an owned slice"] # [doc = " let slice: OwnedSlice = slice_owned(vec, |v| &v[..]);"] # [doc = " assert_eq!(&*slice, [1, 2, 3, 4]);"] # [doc = ""] # [doc = " let slice = slice.slice(|slice| &slice[1..][..2]);"] # [doc = " assert_eq!(&*slice, [2, 3]);"] # [doc = " ```"] # [doc = ""] pub fn slice (self , slicer : impl FnOnce (& [u8]) -> & [u8]) -> OwnedSlice { let bytes = slicer (& self) ; OwnedSlice { bytes , .. self } } }
    };
}

impl_343!()