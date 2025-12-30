// Generated macro for impl_534 (impl)
macro_rules! Depcrate_envimpl_534 {
() => {
// Module: crate::env
// Provides: {"impl_534"}
// Dependencies: {}
# [stable (feature = "env" , since = "1.0.0")] impl Iterator for Vars { type Item = (String , String) ; fn next (& mut self) -> Option < (String , String) > { self . inner . next () . map (| (a , b) | (a . into_string () . unwrap () , b . into_string () . unwrap ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
