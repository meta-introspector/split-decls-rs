// Generated macro for impl_39 (impl)
macro_rules! Depcrate_inline_arrayimpl_39 {
() => {
// Module: crate::inline_array
// Provides: {"impl_39"}
// Dependencies: {}
impl < A , T , Slice > PartialEq < Slice > for InlineArray < A , T > where Slice : Borrow < [A] > , A : PartialEq , { fn eq (& self , other : & Slice) -> bool { self . deref () == other . borrow () } }
};
}
