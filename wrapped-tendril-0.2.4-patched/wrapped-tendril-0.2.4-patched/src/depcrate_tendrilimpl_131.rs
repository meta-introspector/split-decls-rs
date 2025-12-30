// Generated macro for impl_131 (impl)
macro_rules! Depcrate_tendrilimpl_131 {
() => {
// Module: crate::tendril
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a , A > Extend < & 'a [u8] > for Tendril < fmt :: Bytes , A > where A : Atomicity , { # [inline] fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = & 'a [u8] > , { for s in iterable { self . push_slice (s) ; } } }
};
}
