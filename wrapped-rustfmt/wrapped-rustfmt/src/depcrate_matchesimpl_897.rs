// Generated macro for impl_897 (impl)
macro_rules! Depcrate_matchesimpl_897 {
() => {
// Module: crate::matches
// Provides: {"impl_897"}
// Dependencies: {}
impl < 'a > Rewrite for ArmWrapper < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { rewrite_match_arm (context , self . arm , shape , self . is_last , self . beginning_vert . is_some () ,) } }
};
}
