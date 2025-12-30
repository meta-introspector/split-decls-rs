// Generated macro for impl_123 (impl)
macro_rules! Depcrate_tendrilimpl_123 {
() => {
// Module: crate::tendril
// Provides: {"impl_123"}
// Dependencies: {}
impl < A > Extend < char > for Tendril < fmt :: UTF8 , A > where A : Atomicity , { # [inline] fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = char > , { let iterator = iterable . into_iter () ; self . force_reserve (iterator . size_hint () . 0 as u32) ; for c in iterator { self . push_char (c) ; } } }
};
}
