// Generated macro for impl_132 (impl)
macro_rules! Depcrate_database_implimpl_132 {
() => {
// Module: crate::database_impl
// Provides: {"impl_132"}
// Dependencies: {}
impl Default for DatabaseImpl { fn default () -> Self { Self { storage : Storage :: new (if tracing :: enabled ! (Level :: DEBUG) { Some (Box :: new (| event | { crate :: tracing :: debug ! ("salsa_event({:?})" , event) })) } else { None }) , } } }
};
}
