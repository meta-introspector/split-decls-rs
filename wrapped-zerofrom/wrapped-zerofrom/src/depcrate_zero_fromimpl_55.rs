// Generated macro for impl_55 (impl)
macro_rules! Depcrate_zero_fromimpl_55 {
() => {
// Module: crate::zero_from
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'zf , C , T : ZeroFrom < 'zf , C > > ZeroFrom < 'zf , Option < C > > for Option < T > { fn zero_from (other : & 'zf Option < C >) -> Self { other . as_ref () . map (| c | < T as ZeroFrom < C > > :: zero_from (c)) } }
};
}
