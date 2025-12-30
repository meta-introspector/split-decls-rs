// Generated macro for impl_34 (impl)
macro_rules! Depcrate_builder_konst_storeimpl_34 {
() => {
// Module: crate::builder::konst::store
// Provides: {"impl_34"}
// Dependencies: {}
impl < const N : usize > ConstArrayBuilder < N , u8 > { # [doc = " Specialized function that performs `self[index] |= bits`"] # [allow (clippy :: indexing_slicing)] pub (crate) const fn const_bitor_assign_or_panic (mut self , index : usize , bits : u8) -> Self { self . full_array [self . start + index] |= bits ; self } }
};
}
