// Generated macro for THREAD_LOCAL_DESTROYED_ERROR (const)
macro_rules! Depcrate_util_errorTHREAD_LOCAL_DESTROYED_ERROR {
() => {
// Module: crate::util::error
// Provides: {"THREAD_LOCAL_DESTROYED_ERROR"}
// Dependencies: {}
# [doc = " Error string explaining that the Tokio context is not available because the"] # [doc = " thread-local storing it has been destroyed. This usually only happens during"] # [doc = " destructors of other thread-locals."] pub (crate) const THREAD_LOCAL_DESTROYED_ERROR : & str = "The Tokio context thread-local variable has been destroyed." ;
};
}
