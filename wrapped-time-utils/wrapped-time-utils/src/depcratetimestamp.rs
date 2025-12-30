// Generated macro for timestamp (function)
macro_rules! Depcratetimestamp {
() => {
// Module: crate
// Provides: {"timestamp"}
// Dependencies: {}
# [doc = " return timestamp as ms"] pub fn timestamp () -> u64 { SystemTime :: now () . duration_since (UNIX_EPOCH) . expect ("create timestamp in timing") . as_millis () as u64 }
};
}
