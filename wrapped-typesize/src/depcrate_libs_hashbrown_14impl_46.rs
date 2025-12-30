// Generated macro for impl_46 (impl)
macro_rules! Depcrate_libs_hashbrown_14impl_46 {
() => {
// Module: crate::libs::hashbrown_14
// Provides: {"impl_46"}
// Dependencies: {}
impl < K : TypeSize , V : TypeSize , S > TypeSize for HashMap < K , V , S > { fn extra_size (& self) -> usize { generic_map_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
