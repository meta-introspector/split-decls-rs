// Generated macro for ends_with_slash (function)
macro_rules! Depcrate_headerends_with_slash {
() => {
// Module: crate::header
// Provides: {"ends_with_slash"}
// Dependencies: {}
# [cfg (all (unix , not (target_arch = "wasm32")))] fn ends_with_slash (p : & Path) -> bool { p . as_os_str () . as_bytes () . ends_with (b"/") }
};
}
