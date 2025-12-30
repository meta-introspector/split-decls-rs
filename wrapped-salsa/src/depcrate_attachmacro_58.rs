// Generated macro for macro_58 (macro)
macro_rules! Depcrate_attachmacro_58 {
() => {
// Module: crate::attach
// Provides: {"macro_58"}
// Dependencies: {}
# [cfg (not (feature = "shuttle"))] crate :: sync :: thread_local ! { # [doc = " The thread-local state salsa requires for a given thread"] static ATTACHED : Attached = const { Attached :: new () } }
};
}
