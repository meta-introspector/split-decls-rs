// Generated macro for impl_109 (impl)
macro_rules! Depcrate_indeximpl_109 {
() => {
// Module: crate::index
// Provides: {"impl_109"}
// Dependencies: {}
impl < I > ops :: IndexMut < I > for Item where I : Index , { fn index_mut (& mut self , index : I) -> & mut Self { index . index_mut (self) . expect ("index not found") } }
};
}
