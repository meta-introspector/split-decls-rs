// Generated macro for mark_unblocked (function)
macro_rules! Depcrate_registrymark_unblocked {
() => {
// Module: crate::registry
// Provides: {"mark_unblocked"}
// Dependencies: {}
# [doc = " Mark a previously blocked Rayon worker thread as unblocked"] # [inline] pub fn mark_unblocked (registry : & Registry) { registry . sleep . mark_unblocked () }
};
}
