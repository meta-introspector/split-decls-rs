// Generated macro for impl_48 (impl)
macro_rules! Depcrate_libs_hashbrown_14impl_48 {
() => {
// Module: crate::libs::hashbrown_14
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for HashTable < T > { fn extra_size (& self) -> usize { generic_vec_extra_size (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
