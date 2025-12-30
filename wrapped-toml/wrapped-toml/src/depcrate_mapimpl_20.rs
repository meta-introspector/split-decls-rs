// Generated macro for impl_20 (impl)
macro_rules! Depcrate_mapimpl_20 {
() => {
// Module: crate::map
// Provides: {"impl_20"}
// Dependencies: {}
impl < K : Eq + Hash , V : PartialEq > PartialEq for Map < K , V > { # [inline] fn eq (& self , other : & Self) -> bool { self . map . eq (& other . map) } }
};
}
