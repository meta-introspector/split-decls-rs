// Generated macro for impl_24 (impl)
macro_rules! Depcrate_libs_bitvecimpl_24 {
() => {
// Module: crate::libs::bitvec
// Provides: {"impl_24"}
// Dependencies: {}
impl < T : BitStore , O : BitOrder > TypeSize for BitVec < T , O > { fn extra_size (& self) -> usize { div_ceil (self . capacity () , bitvec :: mem :: bits_of :: < T > ()) } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
