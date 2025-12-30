// Generated macro for impl_79 (impl)
macro_rules! Depcrate_map_borrowedimpl_79 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a , K , V > ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K :: Slice : 'static , V :: Slice : 'static , K : ? Sized , V : ? Sized , { # [doc = " Creates a new, empty `ZeroMapBorrowed<K, V>`."] # [doc = ""] # [doc = " Note: Since [`ZeroMapBorrowed`] is not mutable, the return value will be a stub unless"] # [doc = " converted into a [`ZeroMap`](super::ZeroMap)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::maps::ZeroMapBorrowed;"] # [doc = ""] # [doc = " let zm: ZeroMapBorrowed<u16, str> = ZeroMapBorrowed::new();"] # [doc = " assert!(zm.is_empty());"] # [doc = " ```"] pub fn new () -> Self { Self { keys : K :: Container :: zvl_new_borrowed () , values : V :: Container :: zvl_new_borrowed () , } } }
};
}
