// Generated macro for impl_967 (impl)
macro_rules! Depcrate_modules_visitorimpl_967 {
() => {
// Module: crate::modules::visitor
// Provides: {"impl_967"}
// Dependencies: {}
impl < 'a > Drop for FmtVisitor < 'a > { fn drop (& mut self) { if let Some (ctx) = self . parent_context { if self . macro_rewrite_failure { ctx . macro_rewrite_failure . replace (true) ; } } } }
};
}
