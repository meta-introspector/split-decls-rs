// Generated macro for family_name (function)
macro_rules! Depcratefamily_name {
() => {
// Module: crate
// Provides: {"family_name"}
// Dependencies: {}
fn family_name () -> & 'static str { if cfg ! (windows) { "windows" } else { "unix" } }
};
}
