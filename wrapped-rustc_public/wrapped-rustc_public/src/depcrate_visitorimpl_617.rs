// Generated macro for impl_617 (impl)
macro_rules! Depcrate_visitorimpl_617 {
() => {
// Module: crate::visitor
// Provides: {"impl_617"}
// Dependencies: {}
impl < T : Visitable > Visitable for Binder < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . value . visit (visitor) } }
};
}
