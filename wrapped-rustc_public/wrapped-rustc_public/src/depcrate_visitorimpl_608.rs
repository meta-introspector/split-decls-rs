// Generated macro for impl_608 (impl)
macro_rules! Depcrate_visitorimpl_608 {
() => {
// Module: crate::visitor
// Provides: {"impl_608"}
// Dependencies: {}
impl Visitable for UnevaluatedConst { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { let UnevaluatedConst { def , args , promoted } = self ; def . visit (visitor) ? ; args . visit (visitor) ? ; promoted . visit (visitor) } }
};
}
