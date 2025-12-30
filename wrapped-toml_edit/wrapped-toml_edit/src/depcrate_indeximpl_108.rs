// Generated macro for impl_108 (impl)
macro_rules! Depcrate_indeximpl_108 {
() => {
// Module: crate::index
// Provides: {"impl_108"}
// Dependencies: {}
impl < I > ops :: Index < I > for Item where I : Index , { type Output = Self ; fn index (& self , index : I) -> & Self { index . index (self) . expect ("index not found") } }
};
}
