// Generated macro for impl_616 (impl)
macro_rules! Depcrate_visitorimpl_616 {
() => {
// Module: crate::visitor
// Provides: {"impl_616"}
// Dependencies: {}
impl < T : Visitable > Visitable for Vec < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { for arg in self { arg . visit (visitor) ? ; } ControlFlow :: Continue (()) } }
};
}
