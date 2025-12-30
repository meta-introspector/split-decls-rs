// Generated macro for impl_173 (impl)
macro_rules! Depcrate_vecimpl_173 {
() => {
// Module: crate::vec
// Provides: {"impl_173"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for Vec < T > { fn extra_size (& self) -> usize { generic_vec_extra_size :: < T > (self . iter () , self . capacity () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
