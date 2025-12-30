// Generated macro for impl_632 (impl)
macro_rules! Depcrate_visitorimpl_632 {
() => {
// Module: crate::visitor
// Provides: {"impl_632"}
// Dependencies: {}
impl < T : Visitable > Visitable for Binder < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . value . visit (visitor) } }
};
}
