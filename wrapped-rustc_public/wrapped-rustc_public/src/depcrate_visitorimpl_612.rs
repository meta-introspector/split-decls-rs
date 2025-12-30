// Generated macro for impl_612 (impl)
macro_rules! Depcrate_visitorimpl_612 {
() => {
// Module: crate::visitor
// Provides: {"impl_612"}
// Dependencies: {}
impl Visitable for GenericArgs { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . 0 . visit (visitor) } }
};
}
