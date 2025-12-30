// Generated macro for cwd_to_workspace_root (function)
macro_rules! Depcratecwd_to_workspace_root {
() => {
// Module: crate
// Provides: {"cwd_to_workspace_root"}
// Dependencies: {}
# [doc = " Change to workspace root."] # [doc = ""] # [doc = " Assumed this xtask is located in `[WORKSPACE]/crates/xtask-build-man`."] fn cwd_to_workspace_root () -> io :: Result < () > { let pkg_root = std :: env ! ("CARGO_MANIFEST_DIR") ; let ws_root = format ! ("{pkg_root}/../..") ; std :: env :: set_current_dir (ws_root) }
};
}
