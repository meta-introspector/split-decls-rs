// Generated macro for impl_400 (impl)
macro_rules! Depcrate_value_analysisimpl_400 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_400"}
// Dependencies: {}
impl < V : JoinSemiLattice + Clone > JoinSemiLattice for StateData < V > { fn join (& mut self , other : & Self) -> bool { let mut changed = false ; # [allow (rustc :: potential_query_instability)] for (i , v) in other . map . iter () { match self . map . entry (* i) { StdEntry :: Vacant (e) => { e . insert (v . clone ()) ; changed = true } StdEntry :: Occupied (e) => changed |= e . into_mut () . join (v) , } } changed } }
};
}
