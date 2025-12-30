// Generated macro for DebugWithAdapter (struct)
macro_rules! Depcrate_framework_fmtDebugWithAdapter {
() => {
// Module: crate::framework::fmt
// Provides: {"DebugWithAdapter"}
// Dependencies: {}
# [doc = " Implements `fmt::Debug` by deferring to `<T as DebugWithContext<C>>::fmt_with`."] pub struct DebugWithAdapter < 'a , T , C > { pub this : T , pub ctxt : & 'a C , }
};
}
