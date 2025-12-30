// Generated macro for ConstSlice (struct)
macro_rules! Depcrate_builder_konst_storeConstSlice {
() => {
// Module: crate::builder::konst::store
// Provides: {"ConstSlice"}
// Dependencies: {}
# [doc = " A const-friendly slice type. It is backed by a full slice but is primarily intended"] # [doc = " to represent subslices of the full slice. We need this only because we can't take"] # [doc = " subslices in const Rust."] # [derive (Debug , Copy , Clone)] pub (crate) struct ConstSlice < 'a , T > { # [doc = " The full slice."] full_slice : & 'a [T] , # [doc = " The start index of the slice represented by this [`ConstSlice`]."] start : usize , # [doc = " The non-inclusive end index of the slice represented by this [`ConstSlice`]."] limit : usize , }
};
}
