// Generated macro for impl_78 (impl)
macro_rules! Depcrate_libs_serde_jsonimpl_78 {
() => {
// Module: crate::libs::serde_json
// Provides: {"impl_78"}
// Dependencies: {}
impl TypeSize for serde_json :: Map < String , serde_json :: Value > { fn extra_size (& self) -> usize { crate :: map :: generic_map_extra_size (self . iter () , self . len () , self . len ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
