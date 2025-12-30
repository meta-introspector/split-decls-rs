// Generated macro for impl_128 (impl)
macro_rules! Depcrate_inline_tableimpl_128 {
() => {
// Module: crate::inline_table
// Provides: {"impl_128"}
// Dependencies: {}
impl < K : Into < Key > , V : Into < Value > > FromIterator < (K , V) > for InlineTable { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut table = Self :: new () ; table . extend (iter) ; table } }
};
}
