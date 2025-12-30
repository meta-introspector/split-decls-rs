// Generated macro for get_now (function)
macro_rules! Depcrate_windows_systemget_now {
() => {
// Module: crate::windows::system
// Provides: {"get_now"}
// Dependencies: {}
fn get_now () -> u64 { SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) . map (| n | n . as_secs ()) . unwrap_or (0) }
};
}
