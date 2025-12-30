// Generated macro for impl_635 (impl)
macro_rules! Depcrate_visitorimpl_635 {
() => {
// Module: crate::visitor
// Provides: {"impl_635"}
// Dependencies: {}
impl Visitable for FnSig { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . inputs_and_output . visit (visitor) } }
};
}
