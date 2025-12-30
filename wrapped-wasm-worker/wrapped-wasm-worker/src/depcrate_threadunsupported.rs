// Generated macro for unsupported (module)
macro_rules! Depcrate_threadunsupported {
() => {
// Module: crate::thread
// Provides: {"unsupported"}
// Dependencies: {}
# [cfg (not (target_feature = "atomics"))] mod unsupported ;
};
}
