// Generated macro for impl_78 (impl)
macro_rules! Depcrate_map_borrowedimpl_78 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a , K , V > Default for ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K :: Slice : 'static , V :: Slice : 'static , K : ? Sized , V : ? Sized , { fn default () -> Self { Self :: new () } }
};
}
