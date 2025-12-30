// Generated macro for ErrorCounter (struct)
macro_rules! Depcrate_non_blockingErrorCounter {
() => {
// Module: crate::non_blocking
// Provides: {"ErrorCounter"}
// Dependencies: {}
# [doc = " Tracks the number of times a log line was dropped by the background thread."] # [doc = ""] # [doc = " If the non-blocking writer is not configured in [lossy mode], the error"] # [doc = " count should always be 0."] # [doc = ""] # [doc = " [lossy mode]: NonBlockingBuilder::lossy"] # [derive (Clone , Debug)] pub struct ErrorCounter (Arc < AtomicUsize >) ;
};
}
