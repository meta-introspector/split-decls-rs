// Generated macro for impl_404 (impl)
macro_rules! Depcrate_value_analysisimpl_404 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_404"}
// Dependencies: {}
impl < V : JoinSemiLattice + Clone > JoinSemiLattice for State < V > { fn join (& mut self , other : & Self) -> bool { match (& mut * self , other) { (_ , State :: Unreachable) => false , (State :: Unreachable , _) => { * self = other . clone () ; true } (State :: Reachable (this) , State :: Reachable (other)) => this . join (other) , } } }
};
}
