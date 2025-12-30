// Generated macro for get_now (function)
macro_rules! Depcrate_unix_apple_systemget_now {
() => {
// Module: crate::unix::apple::system
// Provides: {"get_now"}
// Dependencies: {}
# [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] fn get_now () -> u64 { SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) . map (| n | n . as_secs ()) . unwrap_or (0) }
};
}
