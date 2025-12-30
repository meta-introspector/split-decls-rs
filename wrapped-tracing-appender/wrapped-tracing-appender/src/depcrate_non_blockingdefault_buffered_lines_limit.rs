// Generated macro for DEFAULT_BUFFERED_LINES_LIMIT (const)
macro_rules! Depcrate_non_blockingDEFAULT_BUFFERED_LINES_LIMIT {
() => {
// Module: crate::non_blocking
// Provides: {"DEFAULT_BUFFERED_LINES_LIMIT"}
// Dependencies: {}
# [doc = " The default maximum number of buffered log lines."] # [doc = ""] # [doc = " If [`NonBlocking`] is lossy, it will drop spans/events at capacity."] # [doc = " If [`NonBlocking`] is _not_ lossy, backpressure will be exerted on"] # [doc = " senders, causing them to block their respective threads until there"] # [doc = " is available capacity."] # [doc = ""] # [doc = " Recommended to be a power of 2."] pub const DEFAULT_BUFFERED_LINES_LIMIT : usize = 128_000 ;
};
}
