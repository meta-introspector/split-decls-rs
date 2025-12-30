// Generated macro for in_ci (function)
macro_rules! Depcrate_ciin_ci {
() => {
// Module: crate::ci
// Provides: {"in_ci"}
// Dependencies: {}
fn in_ci () -> bool { std :: env :: var_os ("CI") == Some ("true" . into ()) }
};
}
