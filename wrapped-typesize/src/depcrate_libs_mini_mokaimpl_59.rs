// Generated macro for impl_59 (impl)
macro_rules! Depcrate_libs_mini_mokaimpl_59 {
() => {
// Module: crate::libs::mini_moka
// Provides: {"impl_59"}
// Dependencies: {}
impl < K : TypeSize + Hash + Eq + PartialEq , V : TypeSize , S : BuildHasher + Clone > TypeSize for mini_moka :: unsync :: Cache < K , V , S > { fn extra_size (& self) -> usize { generic_map_extra_size :: < K , V > (self . iter () , self . entry_count () as usize , self . entry_count () as usize ,) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . entry_count () as usize) } }
};
}
