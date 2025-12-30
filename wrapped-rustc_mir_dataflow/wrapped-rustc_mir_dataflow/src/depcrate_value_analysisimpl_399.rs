// Generated macro for impl_399 (impl)
macro_rules! Depcrate_value_analysisimpl_399 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_399"}
// Dependencies: {}
impl < V : Clone > Clone for StateData < V > { fn clone (& self) -> Self { StateData { bottom : self . bottom . clone () , map : self . map . clone () } } fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) } }
};
}
