// Generated macro for impl_127 (impl)
macro_rules! Depcrate_tendrilimpl_127 {
() => {
// Module: crate::tendril
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a , A > Extend < & 'a u8 > for Tendril < fmt :: Bytes , A > where A : Atomicity , { # [inline] fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = & 'a u8 > , { let iterator = iterable . into_iter () ; self . force_reserve (iterator . size_hint () . 0 as u32) ; for & b in iterator { self . push_slice (& [b]) ; } } }
};
}
