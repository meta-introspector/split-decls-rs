// Generated macro for impl_54 (impl)
macro_rules! Depcrate_libs_hashbrown_15impl_54 {
() => {
// Module: crate::libs::hashbrown_15
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : Eq + Hash + TypeSize , S : BuildHasher > TypeSize for HashSet < T , S > { fn extra_size (& self) -> usize { self . allocation_size () + self . iter () . map (T :: extra_size) . sum :: < usize > () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
