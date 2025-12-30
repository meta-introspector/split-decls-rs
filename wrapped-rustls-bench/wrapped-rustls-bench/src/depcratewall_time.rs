// Generated macro for wall_time (function)
macro_rules! Depcratewall_time {
() => {
// Module: crate
// Provides: {"wall_time"}
// Dependencies: {}
fn wall_time () -> f64 { duration_nanos (SystemTime :: now () . duration_since (UNIX_EPOCH) . unwrap () ,) }
};
}
