// Generated macro for has_been_set (function)
macro_rules! Depcrate_dispatcherhas_been_set {
() => {
// Module: crate::dispatcher
// Provides: {"has_been_set"}
// Dependencies: {}
# [doc = " Returns true if a `tracing` dispatcher has ever been set."] # [doc = ""] # [doc = " This may be used to completely elide trace points if tracing is not in use"] # [doc = " at all or has yet to be initialized."] # [doc (hidden)] # [inline (always)] pub fn has_been_set () -> bool { EXISTS . load (Ordering :: Relaxed) }
};
}
