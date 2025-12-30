// Generated macro for impl_133 (impl)
macro_rules! Depcrate_tendrilimpl_133 {
() => {
// Module: crate::tendril
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'a , F , A > Extend < & 'a Tendril < F , A > > for Tendril < F , A > where F : fmt :: Format + 'a , A : Atomicity , { # [inline] fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = & 'a Tendril < F , A > > , { for t in iterable { self . push_tendril (t) ; } } }
};
}
