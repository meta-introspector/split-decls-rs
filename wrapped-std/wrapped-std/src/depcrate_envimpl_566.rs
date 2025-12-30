// Generated macro for impl_566 (impl)
macro_rules! Depcrate_envimpl_566 {
() => {
// Module: crate::env
// Provides: {"impl_566"}
// Dependencies: {}
# [stable (feature = "env_iterators" , since = "1.12.0")] impl DoubleEndedIterator for Args { fn next_back (& mut self) -> Option < String > { self . inner . next_back () . map (| s | s . into_string () . unwrap ()) } }
};
}
