// Generated macro for impl_1202 (impl)
macro_rules! Depcrate_rewriteimpl_1202 {
() => {
// Module: crate::rewrite
// Provides: {"impl_1202"}
// Dependencies: {}
impl < T : Rewrite > Rewrite for Box < T > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { (* * self) . rewrite (context , shape) } }
};
}
