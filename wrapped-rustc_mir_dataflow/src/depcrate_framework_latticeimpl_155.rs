// Generated macro for impl_155 (impl)
macro_rules! Depcrate_framework_latticeimpl_155 {
() => {
// Module: crate::framework::lattice
// Provides: {"impl_155"}
// Dependencies: {}
impl < T : JoinSemiLattice + Clone > JoinSemiLattice for MaybeReachable < T > { fn join (& mut self , other : & Self) -> bool { match (& mut * self , & other) { (_ , MaybeReachable :: Unreachable) => false , (MaybeReachable :: Unreachable , _) => { * self = other . clone () ; true } (MaybeReachable :: Reachable (this) , MaybeReachable :: Reachable (other)) => this . join (other) , } } }
};
}
