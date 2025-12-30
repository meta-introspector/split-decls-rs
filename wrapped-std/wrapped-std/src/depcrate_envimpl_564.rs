// Generated macro for impl_564 (impl)
macro_rules! Depcrate_envimpl_564 {
() => {
// Module: crate::env
// Provides: {"impl_564"}
// Dependencies: {}
# [stable (feature = "env" , since = "1.0.0")] impl Iterator for Args { type Item = String ; fn next (& mut self) -> Option < String > { self . inner . next () . map (| s | s . into_string () . unwrap ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
