// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : Clone > Clone for IntoIter < T > { # [allow (clippy :: into_iter_on_ref)] fn clone (& self) -> Self { self . as_slice () . into_iter () . cloned () . collect :: < ThinVec < _ > > () . into_iter () } }
};
}
