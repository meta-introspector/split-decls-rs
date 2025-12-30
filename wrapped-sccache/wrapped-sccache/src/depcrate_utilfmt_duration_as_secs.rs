// Generated macro for fmt_duration_as_secs (function)
macro_rules! Depcrate_utilfmt_duration_as_secs {
() => {
// Module: crate::util
// Provides: {"fmt_duration_as_secs"}
// Dependencies: {}
# [doc = " Format `duration` as seconds with a fractional component."] pub fn fmt_duration_as_secs (duration : & Duration) -> String { format ! ("{}.{:03} s" , duration . as_secs () , duration . subsec_millis ()) }
};
}
