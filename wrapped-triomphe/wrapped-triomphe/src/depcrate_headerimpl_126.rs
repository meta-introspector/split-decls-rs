// Generated macro for impl_126 (impl)
macro_rules! Depcrate_headerimpl_126 {
() => {
// Module: crate::header
// Provides: {"impl_126"}
// Dependencies: {}
impl < T : ? Sized > From < Arc < HeaderSlice < () , T > > > for Arc < T > { fn from (this : Arc < HeaderSlice < () , T > >) -> Self { debug_assert_eq ! (Layout :: for_value ::< HeaderSlice < () , T >> (& this) , Layout :: for_value ::< T > (& this . slice)) ; unsafe { Arc :: from_raw_inner (Arc :: into_raw_inner (this) as _) } } }
};
}
