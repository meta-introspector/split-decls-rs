// Generated macro for impl_55 (impl)
macro_rules! Depcrate_libs_hashbrown_15impl_55 {
() => {
// Module: crate::libs::hashbrown_15
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for HashTable < T > { fn extra_size (& self) -> usize { self . allocation_size () + self . iter () . map (T :: extra_size) . sum :: < usize > () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
