// Generated macro for impl_583 (impl)
macro_rules! Depcrate_upvarsimpl_583 {
() => {
// Module: crate::upvars
// Provides: {"impl_583"}
// Dependencies: {}
impl CaptureCollector < '_ , '_ > { fn visit_local_use (& mut self , var_id : HirId , span : Span) { if ! self . locals . contains (& var_id) { self . upvars . entry (var_id) . or_insert (hir :: Upvar { span }) ; } } }
};
}
