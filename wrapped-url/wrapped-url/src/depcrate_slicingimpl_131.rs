// Generated macro for impl_131 (impl)
macro_rules! Depcrate_slicingimpl_131 {
() => {
// Module: crate::slicing
// Provides: {"impl_131"}
// Dependencies: {}
impl Index < RangeFrom < Position > > for Url { type Output = str ; fn index (& self , range : RangeFrom < Position >) -> & str { & self . serialization [self . index (range . start) ..] } }
};
}
