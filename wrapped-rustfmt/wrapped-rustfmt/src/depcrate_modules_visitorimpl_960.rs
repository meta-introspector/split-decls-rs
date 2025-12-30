// Generated macro for impl_960 (impl)
macro_rules! Depcrate_modules_visitorimpl_960 {
() => {
// Module: crate::modules::visitor
// Provides: {"impl_960"}
// Dependencies: {}
impl < 'a > Drop for FmtVisitor < 'a > { fn drop (& mut self) { if let Some (ctx) = self . parent_context { if self . macro_rewrite_failure { ctx . macro_rewrite_failure . replace (true) ; } } } }
};
}
