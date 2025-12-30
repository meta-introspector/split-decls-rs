// Generated macro for impl_41 (impl)
macro_rules! Depcrate_inline_arrayimpl_41 {
() => {
// Module: crate::inline_array
// Provides: {"impl_41"}
// Dependencies: {}
impl < A , T > PartialOrd for InlineArray < A , T > where A : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
