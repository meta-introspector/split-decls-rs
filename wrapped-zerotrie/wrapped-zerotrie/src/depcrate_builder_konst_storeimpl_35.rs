// Generated macro for impl_35 (impl)
macro_rules! Depcrate_builder_konst_storeimpl_35 {
() => {
// Module: crate::builder::konst::store
// Provides: {"impl_35"}
// Dependencies: {}
impl < const N : usize , T : Copy > ConstArrayBuilder < N , T > { # [doc = " Swaps the elements at positions `i` and `j`."] # [cfg (feature = "alloc")] pub fn swap_or_panic (mut self , i : usize , j : usize) -> Self { self . full_array . swap (self . start + i , self . start + j) ; self } }
};
}
