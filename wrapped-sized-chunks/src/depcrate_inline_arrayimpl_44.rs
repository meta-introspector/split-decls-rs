// Generated macro for impl_44 (impl)
macro_rules! Depcrate_inline_arrayimpl_44 {
() => {
// Module: crate::inline_array
// Provides: {"impl_44"}
// Dependencies: {}
impl < A , T > Hash for InlineArray < A , T > where A : Hash , { fn hash < H > (& self , hasher : & mut H) where H : Hasher , { for item in self { item . hash (hasher) } } }
};
}
