// Generated macro for impl_46 (impl)
macro_rules! Depcrate_inline_arrayimpl_46 {
() => {
// Module: crate::inline_array
// Provides: {"impl_46"}
// Dependencies: {}
impl < A , T > FromIterator < A > for InlineArray < A , T > { fn from_iter < I > (it : I) -> Self where I : IntoIterator < Item = A > , { let mut chunk = Self :: new () ; for item in it { chunk . push (item) ; } chunk } }
};
}
