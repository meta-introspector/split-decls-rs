// Generated macro for DebugDiffWithAdapter (struct)
macro_rules! Depcrate_framework_fmtDebugDiffWithAdapter {
() => {
// Module: crate::framework::fmt
// Provides: {"DebugDiffWithAdapter"}
// Dependencies: {}
# [doc = " Implements `fmt::Debug` by deferring to `<T as DebugWithContext<C>>::fmt_diff_with`."] pub struct DebugDiffWithAdapter < 'a , T , C > { pub new : T , pub old : T , pub ctxt : & 'a C , }
};
}
