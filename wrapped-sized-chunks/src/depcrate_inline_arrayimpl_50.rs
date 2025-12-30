// Generated macro for impl_50 (impl)
macro_rules! Depcrate_inline_arrayimpl_50 {
() => {
// Module: crate::inline_array
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , A , T > Extend < & 'a A > for InlineArray < A , T > where A : 'a + Copy , { # [doc = " Append the contents of the iterator to the back of the array."] # [doc = ""] # [doc = " Panics if the array exceeds its capacity."] # [doc = ""] # [doc = " Time: O(n) for the length of the iterator"] fn extend < I > (& mut self , it : I) where I : IntoIterator < Item = & 'a A > , { for item in it { self . push (* item) ; } } }
};
}
