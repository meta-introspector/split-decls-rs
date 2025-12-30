// Generated macro for impl_127 (impl)
macro_rules! Depcrate_headerimpl_127 {
() => {
// Module: crate::header
// Provides: {"impl_127"}
// Dependencies: {}
impl < T : ? Sized > From < Arc < T > > for Arc < HeaderSlice < () , T > > { fn from (this : Arc < T >) -> Self { unsafe { Arc :: from_raw_inner (Arc :: into_raw_inner (this) as _) } } }
};
}
