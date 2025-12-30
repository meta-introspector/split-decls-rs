// Generated macro for impl_32 (impl)
macro_rules! Depcrate_inline_arrayimpl_32 {
() => {
// Module: crate::inline_array
// Provides: {"impl_32"}
// Dependencies: {}
impl < A , T > Clone for InlineArray < A , T > where A : Clone , { fn clone (& self) -> Self { let mut copy = Self :: new () ; for i in 0 .. self . len () { unsafe { copy . write_at (i , self . get_unchecked (i) . clone ()) ; } } unsafe { * copy . len_mut () = self . len () ; } copy } }
};
}
