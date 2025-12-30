// Generated macro for impl_129 (impl)
macro_rules! Depcrate_tendrilimpl_129 {
() => {
// Module: crate::tendril
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a , A > Extend < & 'a str > for Tendril < fmt :: UTF8 , A > where A : Atomicity , { # [inline] fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = & 'a str > , { for s in iterable { self . push_slice (s) ; } } }
};
}
