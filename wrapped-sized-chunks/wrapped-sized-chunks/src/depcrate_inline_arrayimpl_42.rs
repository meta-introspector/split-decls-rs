// Generated macro for impl_42 (impl)
macro_rules! Depcrate_inline_arrayimpl_42 {
() => {
// Module: crate::inline_array
// Provides: {"impl_42"}
// Dependencies: {}
impl < A , T > Ord for InlineArray < A , T > where A : Ord , { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
