// Generated macro for impl_106 (impl)
macro_rules! Depcrate_primitivesimpl_106 {
() => {
// Module: crate::primitives
// Provides: {"impl_106"}
// Dependencies: {}
impl < const N : usize , T : TypeSize > TypeSize for [T ; N] { fn extra_size (& self) -> usize { self . iter () . map (T :: extra_size) . sum () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (N) } }
};
}
