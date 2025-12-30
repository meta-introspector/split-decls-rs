// Generated macro for pty (module)
macro_rules! Depcratepty {
() => {
// Module: crate
// Provides: {"pty"}
// Dependencies: {}
# [cfg (not (windows))] # [cfg (not (target_os = "wasi"))] # [cfg (feature = "pty")] # [cfg_attr (docsrs , doc (cfg (feature = "pty")))] pub mod pty ;
};
}
