// Generated macro for impl_627 (impl)
macro_rules! Depcrate_visitorimpl_627 {
() => {
// Module: crate::visitor
// Provides: {"impl_627"}
// Dependencies: {}
impl Visitable for GenericArgs { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . 0 . visit (visitor) } }
};
}
