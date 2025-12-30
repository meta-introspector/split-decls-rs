// Generated macro for impl_402 (impl)
macro_rules! Depcrate_value_analysisimpl_402 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_402"}
// Dependencies: {}
impl < V : Clone > Clone for State < V > { fn clone (& self) -> Self { match self { Self :: Reachable (x) => Self :: Reachable (x . clone ()) , Self :: Unreachable => Self :: Unreachable , } } fn clone_from (& mut self , source : & Self) { match (& mut * self , source) { (Self :: Reachable (x) , Self :: Reachable (y)) => { x . clone_from (& y) ; } _ => * self = source . clone () , } } }
};
}
