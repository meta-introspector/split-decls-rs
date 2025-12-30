// Generated macro for impl_35 (impl)
macro_rules! Depcrate_libs_dashmapimpl_35 {
() => {
// Module: crate::libs::dashmap
// Provides: {"impl_35"}
// Dependencies: {}
impl < K : TypeSize , V : TypeSize , S > TypeSize for DashMap < K , V , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn extra_size (& self) -> usize { self . shards () . iter () . map (TypeSize :: get_size) . sum :: < usize > () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
