// Generated macro for impl_49 (impl)
macro_rules! Depcrate_inline_arrayimpl_49 {
() => {
// Module: crate::inline_array
// Provides: {"impl_49"}
// Dependencies: {}
impl < A , T > Extend < A > for InlineArray < A , T > { # [doc = " Append the contents of the iterator to the back of the array."] # [doc = ""] # [doc = " Panics if the array exceeds its capacity."] # [doc = ""] # [doc = " Time: O(n) for the length of the iterator"] fn extend < I > (& mut self , it : I) where I : IntoIterator < Item = A > , { for item in it { self . push (item) ; } } }
};
}
