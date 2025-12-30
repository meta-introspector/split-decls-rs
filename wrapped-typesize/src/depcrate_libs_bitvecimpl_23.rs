// Generated macro for impl_23 (impl)
macro_rules! Depcrate_libs_bitvecimpl_23 {
() => {
// Module: crate::libs::bitvec
// Provides: {"impl_23"}
// Dependencies: {}
impl < A : BitViewSized + TypeSize , O : BitOrder > TypeSize for BitArray < A , O > { fn extra_size (& self) -> usize { self . data . extra_size () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
