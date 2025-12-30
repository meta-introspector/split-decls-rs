// Generated macro for cargo_proxy (function)
macro_rules! Depcratecargo_proxy {
() => {
// Module: crate
// Provides: {"cargo_proxy"}
// Dependencies: {}
# [doc = " Looks up the binary in the cargo home directory if it exists."] fn cargo_proxy (executable_name : & str) -> Option < Utf8PathBuf > { let mut path = get_cargo_home () ? ; path . push ("bin") ; path . push (executable_name) ; probe_for_binary (path) }
};
}
