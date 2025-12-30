// Generated macro for impl_154 (impl)
macro_rules! Depcrate_framework_latticeimpl_154 {
() => {
// Module: crate::framework::lattice
// Provides: {"impl_154"}
// Dependencies: {}
impl < V : Clone > Clone for MaybeReachable < V > { fn clone (& self) -> Self { match self { MaybeReachable :: Reachable (x) => MaybeReachable :: Reachable (x . clone ()) , MaybeReachable :: Unreachable => MaybeReachable :: Unreachable , } } fn clone_from (& mut self , source : & Self) { match (& mut * self , source) { (MaybeReachable :: Reachable (x) , MaybeReachable :: Reachable (y)) => { x . clone_from (y) ; } _ => * self = source . clone () , } } }
};
}
