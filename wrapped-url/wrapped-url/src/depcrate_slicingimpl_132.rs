// Generated macro for impl_132 (impl)
macro_rules! Depcrate_slicingimpl_132 {
() => {
// Module: crate::slicing
// Provides: {"impl_132"}
// Dependencies: {}
impl Index < RangeTo < Position > > for Url { type Output = str ; fn index (& self , range : RangeTo < Position >) -> & str { & self . serialization [.. self . index (range . end)] } }
};
}
