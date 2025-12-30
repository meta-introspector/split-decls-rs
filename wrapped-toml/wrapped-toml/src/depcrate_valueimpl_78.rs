// Generated macro for impl_78 (impl)
macro_rules! Depcrate_valueimpl_78 {
() => {
// Module: crate::value
// Provides: {"impl_78"}
// Dependencies: {}
impl < I > ops :: IndexMut < I > for Value where I : Index , { fn index_mut (& mut self , index : I) -> & mut Self { self . get_mut (index) . expect ("index not found") } }
};
}
