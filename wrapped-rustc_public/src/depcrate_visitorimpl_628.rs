// Generated macro for impl_628 (impl)
macro_rules! Depcrate_visitorimpl_628 {
() => {
// Module: crate::visitor
// Provides: {"impl_628"}
// Dependencies: {}
impl Visitable for Region { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { visitor . visit_reg (self) } fn super_visit < V : Visitor > (& self , _ : & mut V) -> ControlFlow < V :: Break > { ControlFlow :: Continue (()) } }
};
}
