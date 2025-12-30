// Generated macro for impl_133 (impl)
macro_rules! Depcrate_slicingimpl_133 {
() => {
// Module: crate::slicing
// Provides: {"impl_133"}
// Dependencies: {}
impl Index < Range < Position > > for Url { type Output = str ; fn index (& self , range : Range < Position >) -> & str { & self . serialization [self . index (range . start) .. self . index (range . end)] } }
};
}
