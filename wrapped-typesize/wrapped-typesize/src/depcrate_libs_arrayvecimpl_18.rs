// Generated macro for impl_18 (impl)
macro_rules! Depcrate_libs_arrayvecimpl_18 {
() => {
// Module: crate::libs::arrayvec
// Provides: {"impl_18"}
// Dependencies: {}
impl < T : TypeSize , const CAP : usize > TypeSize for ArrayVec < T , CAP > { fn extra_size (& self) -> usize { self . iter () . map (TypeSize :: extra_size) . sum () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
