// Generated macro for impl_45 (impl)
macro_rules! Depcrate_panic_contextimpl_45 {
() => {
// Module: crate::panic_context
// Provides: {"impl_45"}
// Dependencies: {}
impl Drop for PanicContext { fn drop (& mut self) { with_ctx (| ctx | assert ! (ctx . pop () . is_some ())) ; } }
};
}
