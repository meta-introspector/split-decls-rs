// Generated macro for version (function)
macro_rules! Depcrateversion {
() => {
// Module: crate
// Provides: {"version"}
// Dependencies: {}
pub fn version () -> String { let mut v = env ! ("CARGO_PKG_VERSION") . to_string () ; if let Some (s) = option_env ! ("WBG_VERSION") { v . push_str (" (") ; v . push_str (s) ; v . push (')') ; } v }
};
}
