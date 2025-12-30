// Generated macro for impl_206 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_206 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_206"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 :: Slice : 'static , K1 :: Slice : 'static , V :: Slice : 'static , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { # [doc = " Creates a new, empty `ZeroMap2dBorrowed<K0, K1, V>`."] # [doc = ""] # [doc = " Note: Since [`ZeroMap2dBorrowed`] is not mutable, the return value will be a stub unless"] # [doc = " converted into a [`ZeroMap2d`](super::ZeroMap2d)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::maps::ZeroMap2dBorrowed;"] # [doc = ""] # [doc = " let zm: ZeroMap2dBorrowed<u16, u16, str> = ZeroMap2dBorrowed::new();"] # [doc = " assert!(zm.is_empty());"] # [doc = " ```"] pub fn new () -> Self { Self { keys0 : K0 :: Container :: zvl_new_borrowed () , joiner : Default :: default () , keys1 : K1 :: Container :: zvl_new_borrowed () , values : V :: Container :: zvl_new_borrowed () , } } }
};
}
