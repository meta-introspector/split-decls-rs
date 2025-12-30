// Generated macro for impl_47 (impl)
macro_rules! Depcrate_libs_hashbrown_14impl_47 {
() => {
// Module: crate::libs::hashbrown_14
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : TypeSize , S > TypeSize for HashSet < T , S > { fn extra_size (& self) -> usize { generic_vec_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
