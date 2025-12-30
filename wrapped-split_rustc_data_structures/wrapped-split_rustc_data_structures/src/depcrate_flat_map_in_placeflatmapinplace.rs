// Generated macro for FlatMapInPlace (trait)
macro_rules! Depcrate_flat_map_in_placeFlatMapInPlace {
() => {
// Module: crate::flat_map_in_place
// Provides: {"FlatMapInPlace"}
// Dependencies: {}
pub trait FlatMapInPlace < T > : Sized { fn flat_map_in_place < F , I > (& mut self , f : F) where F : FnMut (T) -> I , I : IntoIterator < Item = T > ; }
};
}
