// Generated macro for impl_39 (impl)
macro_rules! Depcrate_libs_extract_mapimpl_39 {
() => {
// Module: crate::libs::extract_map
// Provides: {"impl_39"}
// Dependencies: {}
impl < K , V , S > TypeSize for ExtractMap < K , V , S > where V : TypeSize , { fn extra_size (& self) -> usize { generic_vec_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
