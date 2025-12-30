// Generated macro for impl_631 (impl)
macro_rules! Depcrate_visitorimpl_631 {
() => {
// Module: crate::visitor
// Provides: {"impl_631"}
// Dependencies: {}
impl < T : Visitable > Visitable for Vec < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { for arg in self { arg . visit (visitor) ? ; } ControlFlow :: Continue (()) } }
};
}
