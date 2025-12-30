// Generated macro for impl_205 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_205 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > Default for ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 :: Slice : 'static , K1 :: Slice : 'static , V :: Slice : 'static , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { fn default () -> Self { Self :: new () } }
};
}
