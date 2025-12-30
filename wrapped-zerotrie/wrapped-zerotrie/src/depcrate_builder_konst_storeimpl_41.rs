// Generated macro for impl_41 (impl)
macro_rules! Depcrate_builder_konst_storeimpl_41 {
() => {
// Module: crate::builder::konst::store
// Provides: {"impl_41"}
// Dependencies: {}
impl < const K : usize > ConstArrayBuilder < K , BranchMeta > { # [doc = " Converts this builder-array of [`BranchMeta`] to one of the `ascii` fields."] pub const fn map_to_ascii_bytes (& self) -> ConstArrayBuilder < K , u8 > { let mut result = ConstArrayBuilder :: new_empty ([0 ; K] , K) ; let self_as_slice = self . as_const_slice () ; const_for_each ! (self_as_slice , value , { result = result . const_push_front_or_panic (value . ascii) ; }) ; result } }
};
}
